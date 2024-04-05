use diesel::{ExpressionMethods, insert_into, PgConnection, RunQueryDsl};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::upsert::excluded;

use crate::model::database::User;
use crate::schema::users;
use crate::schema::users::{id, planning_center_id};

pub fn to_database(conn: Pool<ConnectionManager<PgConnection>>, data: &Vec<User>) {
    let mut connection = conn.get().expect("Error getting connection");

    let result: Vec<User> = insert_into(users::table)
        .values(data)
        .on_conflict(id)
        .do_update()
        .set(planning_center_id.eq(excluded(planning_center_id)))
        .get_results(&mut connection)
        .expect("Error inserting users")
        ;

    for user in result {
        println!("{:?}", user);
    }
}