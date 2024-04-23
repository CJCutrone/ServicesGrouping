use diesel::{ExpressionMethods, insert_into, PgConnection, RunQueryDsl};
use diesel::upsert::excluded;

use crate::model::database::User;
use crate::schema::users;
use crate::schema::users::{id, planning_center_id};

pub fn to_database(conn: &mut PgConnection, data: &Vec<User>) {
    let result: Vec<User> = insert_into(users::table)
        .values(data)
        .on_conflict(id)
        .do_update()
        .set(planning_center_id.eq(excluded(planning_center_id)))
        .get_results(conn)
        .expect("Error inserting users")
        ;

    for user in result {
        println!("{:?}", user);
    }
}