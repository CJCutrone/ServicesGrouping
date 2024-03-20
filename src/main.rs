use std::env;
use clap::Parser;

use diesel::{Connection, PgConnection};
use dotenv::dotenv;
use crate::commands::{Arguments, Commands};

pub mod model;
pub mod actions;
pub mod schema;

pub mod commands;

fn main() {
    let args = Arguments::parse();

    if let Commands::Update { path } = args.command {
        let mut connection = establish_connection();
        actions::data::process(&*path, &mut connection);
    }
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    return PgConnection::establish(&url).expect(&format!("Error connecting to {}", url))
}