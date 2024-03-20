use std::env;

use diesel::{Connection, PgConnection};
use dotenv::dotenv;
pub mod model;
pub mod actions;
pub mod schema;

fn main() {
    let path = "C:\\Users\\CJ\\Downloads\\test.json";
    let mut connection = establish_connection();

    actions::data::process(path, &mut connection);
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    return PgConnection::establish(&url).expect(&format!("Error connecting to {}", url))
}