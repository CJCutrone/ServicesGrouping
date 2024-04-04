use diesel::{ExpressionMethods, insert_into, PgConnection, RunQueryDsl};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::upsert::excluded;

use crate::model::database::{Group};
use crate::schema::{groups};
use crate::schema::groups::{id, planning_center_id};

pub fn to_database(pool: Pool<ConnectionManager<PgConnection>>, data: &Vec<Group>) {
    let mut connection = pool.get().expect("Error getting connection");

    insert_into(groups::table)
        .values(data)
        .on_conflict(id)
        .do_update()
        .set(planning_center_id.eq(excluded(planning_center_id)))
        .execute(&mut connection)
        .expect("Error inserting users");
}