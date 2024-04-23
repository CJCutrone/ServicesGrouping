use std::io::{Error, ErrorKind};
use std::str::FromStr;
use actix_cors::Cors;
use actix_session::config::PersistentSession;
use actix_session::SessionMiddleware;
use actix_session::storage::CookieSessionStore;
use actix_web::{App, cookie, HttpServer};
use actix_web::cookie::Key;
use actix_web::web::Data;
use chrono::{DateTime, NaiveDateTime, Utc};
use clap::Parser;
use cron::Schedule;
use log::{error, info, trace};
use uuid::Uuid;
use dotenv::dotenv;

use crate::actions::data::get_db_connection;
use crate::commands::{Arguments, Commands};

pub mod actions;
pub mod api;
pub mod commands;
pub mod model;
pub mod oauth;
pub mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    dotenv().ok();
    pretty_env_logger::init();
    let args = Arguments::parse();

    let pool = get_db_connection();
    if let Err(e) = pool {
        error!("{:?}", e);
        return Err(Error::new(ErrorKind::Other, "Failed to establish connection to database"));
    }
    let pool = pool.unwrap();
    info!("Connection pool established");

    match args.command {
        Commands::Api => {
            info!("API command received");
            return HttpServer::new(move ||
                App::new()
                    .wrap(
                        //TODO: once initial development is done, widdle down what is allowed
                        Cors::default()
                            .allow_any_origin()
                            .allow_any_method()
                            .allow_any_header()
                            .supports_credentials()
                            .max_age(3600)
                    )
                    .wrap(
                        SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
                            .cookie_secure(true)
                            .session_lifecycle(
                                PersistentSession::default().session_ttl(cookie::time::Duration::hours(2)),
                            )
                            .build()
                    )
                    .service(api::endpoints::generate_assignments)
                    .service(api::webhooks::groups::group_created_webhook)
                    .service(api::webhooks::groups::group_updated_webhook)
                    .service(api::webhooks::groups::group_deleted_webhook)
                    .service(api::webhooks::groups::membership::membership_created_webhook)
                    .service(oauth::endpoints::callback)
                    .service(oauth::endpoints::me)
                    .app_data(Data::new(pool.clone()))
                )
                .bind("0.0.0.0:8080")?
                .run()
                .await
        }
        Commands::Update { path } => {
            info!("Update command received");
            actions::data::process(&path, pool);
        }
        Commands::Assign { group_id, start_date, end_date, cron } => {
            info!("Assign command received");
            let group_id = Uuid::parse_str(&group_id).unwrap();
            let start_date = NaiveDateTime::parse_from_str(&start_date, "%Y-%m-%d %H:%M").unwrap();
            let end_date = NaiveDateTime::parse_from_str(&end_date, "%Y-%m-%d %H:%M").unwrap();
            let cron = cron.split(';').collect::<Vec<&str>>();
            trace!("Arguments parsed successfully");
            let mut dates: Vec<NaiveDateTime> = cron
                .iter()
                .flat_map(|c|
                    generate_dates(start_date.and_utc(), end_date.and_utc(), c).unwrap_or_default()
                )
                .collect()
                ;

            dates.sort();

            trace!("Dates generated");
            let _ = actions::data::ticketing::generate_assignments_for_dates(group_id, dates, pool);
        }
    }

    Ok(())
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

    Some(dates)
}