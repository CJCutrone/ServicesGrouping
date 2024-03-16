use std::env;

use diesel::{Connection, PgConnection};
use dotenv::dotenv;

use crate::model::database::{Group};

pub mod model;
pub mod actions;
pub mod schema;

fn main() {
    let path = "C:\\Users\\CJ\\Downloads\\test_groups.json";
    let data = actions::data::load::group::from_json(path);

    //convert excel User to database User
    let data = data.iter().map(|group|
        Group::from_json(group)
    ).collect();


    let mut connection = establish_connection();

    actions::data::save::group::to_database(&mut connection, &data);
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    return PgConnection::establish(&url).expect(&format!("Error connecting to {}", url))
}