use std::env;

use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError};
use log::trace;

pub mod get;
pub mod save;
pub mod ticketing;

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