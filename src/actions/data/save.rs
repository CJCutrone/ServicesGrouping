use diesel::{insert_into, PgConnection, RunQueryDsl};

use crate::model::database::User;
use crate::schema::users;

pub fn to_database(conn: &mut PgConnection, data: &Vec<User>) {
    insert_into(users::table)
        .values(data)
        .execute(conn)
        .expect("Error inserting users");
}