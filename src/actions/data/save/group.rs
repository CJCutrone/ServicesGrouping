use diesel::result::Error;
use diesel::{ExpressionMethods, insert_into, PgConnection, RunQueryDsl};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::upsert::excluded;

use crate::model::database::Group;
use crate::schema::groups;
use crate::schema::groups::{id, name, planning_center_id, positions, is_deleted};

pub fn to_database(pool: Pool<ConnectionManager<PgConnection>>, data: &Vec<Group>) -> Result<usize, Error>{
    let mut connection = pool.get().expect("Error getting connection");

    insert_into(groups::table)
        .values(data)
        .on_conflict(id)
        .do_update()
        .set((
            planning_center_id.eq(excluded(planning_center_id)),
            name.eq(excluded(name)),
            positions.eq(excluded(positions)),
            is_deleted.eq(excluded(is_deleted))
        ))
        .execute(&mut connection)
}