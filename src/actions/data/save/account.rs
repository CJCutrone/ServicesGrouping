use diesel::{ExpressionMethods, insert_into, PgConnection, RunQueryDsl};
use diesel::upsert::excluded;

use crate::model::database::Account;
use crate::schema::accounts;
use crate::schema::accounts::{access_token, id, planning_center_id, refresh_token};

pub fn to_database(conn: &mut PgConnection, data: Account) {
    let _ = insert_into(accounts::table)
        .values(vec![data])
        .on_conflict(id)
        .do_update()
        .set((
            planning_center_id.eq(excluded(planning_center_id)),
            access_token.eq(excluded(access_token)),
            refresh_token.eq(excluded(refresh_token))
        ))
        .execute(conn)
        .expect("Error inserting account information")
        ;
}