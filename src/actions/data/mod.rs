use std::env;

use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError};
use log::trace;

pub mod get;
pub mod load;
pub mod save;
pub mod ticketing;

pub fn process(file: &str, pool: Pool<ConnectionManager<PgConnection>>) {
    let users = load::user::from(file);
    let groups = load::group::from(file);
    let group_assignments = load::group_assignment::from(file);

    let mut pool = pool.get().expect("Error getting connection");
    let _  = pool.build_transaction()
        .read_write()
        .run::<_, diesel::result::Error, _>(|conn| {
            trace!("Saving user to database");
            save::user::to_database(conn, &users);
            trace!("Saving groups to database");
            save::group::to_database(conn, &groups).expect("Unable to insert groups into database");
            trace!("Saving group_assignments to database");
            save::group_assignment::to_database(conn, &group_assignments);

            Ok(())
        });
}

pub fn get_db_connection() -> Result<Pool<ConnectionManager<PgConnection>>, PoolError> {
    trace!("Pulling in .env vars");
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    trace!("URL for database found");
    let manager = ConnectionManager::<PgConnection>::new(url);
    trace!("ConnectionManager established");
    let pool = Pool::builder()
        .max_size(1)
        .test_on_check_out(true)
        .build(manager);
    trace!("Pool established");

    pool
}