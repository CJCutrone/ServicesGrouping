use std::env;
use diesel::{Connection, PgConnection};
use dotenv::dotenv;

pub mod load;
pub mod save;
pub mod ticketing;

pub fn process(file: &str, connection: &mut PgConnection) {
    let users = load::user::from(file);
    let groups = load::group::from(file);
    let group_assignments = load::group_assignment::from(file);

    save::user::to_database(connection, &users);
    save::group::to_database(connection, &groups);
    save::group_assignment::to_database(connection, &group_assignments);
}

pub fn get_db_connection() -> PgConnection {
    dotenv().ok();

    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    return PgConnection::establish(&url).expect(&format!("Error connecting to {}", url))
}