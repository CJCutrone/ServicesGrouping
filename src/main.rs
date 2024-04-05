use std::str::FromStr;
use chrono::{DateTime, NaiveDateTime, Utc};
use clap::Parser;
use cron::Schedule;
use log::{error, trace};
use uuid::Uuid;

use crate::actions::data::get_db_connection;
use crate::commands::{Arguments, Commands};

pub mod model;
pub mod actions;
pub mod schema;
pub mod commands;

fn main() {
    pretty_env_logger::init();
    let args = Arguments::parse();
    trace!("Starting CLI");

    match args.command {
        Commands::Update { path } => {
            trace!("Update command received");
            let pool = get_db_connection();
            if let Err(e) = pool {
                error!("{:?}", e);
                return;
            }
            actions::data::process(&*path, pool.unwrap());
        }
        Commands::Assign { group_id, start_date, end_date, cron } => {
            trace!("Assign command received");
            let group_id = Uuid::parse_str(&group_id).unwrap();
            let start_date = NaiveDateTime::parse_from_str(&start_date, "%Y-%m-%d %H:%M").unwrap();
            let end_date = NaiveDateTime::parse_from_str(&end_date, "%Y-%m-%d %H:%M").unwrap();
            let cron = cron.split(";").collect::<Vec<&str>>();

            trace!("Arguments parsed successfully");
            let pool = get_db_connection();
            if let Err(e) = pool {
                error!("{:?}", e);
                return;
            }

            trace!("Connection pool established");
            let mut dates: Vec<NaiveDateTime> = cron
                .iter()
                .map(|c|
                    generate_dates(start_date.and_utc(), end_date.and_utc(), c).unwrap_or_else(|| Vec::new())
                )
                .flatten()
                .collect()
                ;

            dates.sort_by(|a, b| a.cmp(b));

            trace!("Dates generated");
            let _ = actions::data::ticketing::generate_assignments_for_dates(group_id, dates, pool.unwrap());
        }
    }
}

fn generate_dates(
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
    cron: &str
) -> Option<Vec<NaiveDateTime>> {
    let schedule = Schedule::from_str(cron).unwrap();
    let mut next_date = schedule.after(&start_date);

    let mut dates = Vec::new();
    let mut current_date = next_date.next()?;
    while current_date <= end_date {
        dates.push(current_date.naive_utc());
        current_date = next_date.next()?;
    }

    return Some(dates);
}