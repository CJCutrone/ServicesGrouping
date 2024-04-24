
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError};
use log::trace;

pub mod get;
pub mod save;
pub mod ticketing;

pub fn get_db_connection(database_connection: String) -> Result<Pool<ConnectionManager<PgConnection>>, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_connection);
    trace!("ConnectionManager established");
    let pool = Pool::builder()
        .max_size(1)
        .test_on_check_out(true)
        .build(manager);
    trace!("Pool established");

    pool
}