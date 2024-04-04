use std::env;
use diesel::{PgConnection};
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;

pub mod load;
pub mod save;
pub mod ticketing;

pub fn process(file: &str, pool: Pool<ConnectionManager<PgConnection>>) {
    let users = load::user::from(file);
    let groups = load::group::from(file);
    let group_assignments = load::group_assignment::from(file);

    save::user::to_database(pool.clone(), &users);
    save::group::to_database(pool.clone(), &groups);
    save::group_assignment::to_database(pool.clone(), &group_assignments);
}

pub fn get_db_connection() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(url);
    return Pool::builder().build(manager).expect("Failed to create pool");
}