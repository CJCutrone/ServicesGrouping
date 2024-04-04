use diesel::{ExpressionMethods, insert_into, PgConnection, RunQueryDsl};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::upsert::excluded;

use crate::model::database::{GroupAssignment};
use crate::schema::{group_assignments};
use crate::schema::group_assignments::{id, group_id, user_id};

pub fn to_database(pool: Pool<ConnectionManager<PgConnection>>, data: &Vec<GroupAssignment>) {
    let mut connection = pool.get().expect("Error getting connection");
    insert_into(group_assignments::table)
        .values(data)
        .on_conflict(id)
        .do_update()
        .set((
            group_id.eq(excluded(group_id)),
            user_id.eq(excluded(user_id))
        ))
        .execute(&mut connection)
        .expect("Error inserting group assignments");
}