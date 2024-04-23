use diesel::{ExpressionMethods, insert_into, PgConnection, RunQueryDsl};
use diesel::upsert::excluded;

use crate::model::database::GroupAssignment;
use crate::schema::group_assignments;
use crate::schema::group_assignments::{id, group_id, user_id};

pub fn to_database(conn: &mut PgConnection, data: &Vec<GroupAssignment>) {
    insert_into(group_assignments::table)
        .values(data)
        .on_conflict(id)
        .do_update()
        .set((
            group_id.eq(excluded(group_id)),
            user_id.eq(excluded(user_id))
        ))
        .execute(conn)
        .expect("Error inserting group assignments");
}