use diesel::{ExpressionMethods, insert_into, PgConnection, RunQueryDsl};
use diesel::upsert::excluded;
use magic_crypt::{new_magic_crypt, MagicCryptTrait};

use crate::model::database::{self, PlanningCenterAccessTokens};
use crate::oauth::model::AccountTokens;
use crate::schema::planning_center_access_tokens;
use crate::schema::planning_center_access_tokens::{access_token, id, planning_center_id, refresh_token, expires_at};
use crate::ApplicationConfiguration;

pub fn to_database(conn: &mut PgConnection, data: PlanningCenterAccessTokens) {
    let _ = insert_into(planning_center_access_tokens::table)
        .values(vec![data])
        .on_conflict(id)
        .do_update()
        .set((
            planning_center_id.eq(excluded(planning_center_id)),
            access_token.eq(excluded(access_token)),
            refresh_token.eq(excluded(refresh_token)),
            expires_at.eq(excluded(expires_at))
        ))
        .execute(conn)
        .expect("Error inserting account information")
        ;
}

pub fn encrypt_and_store_tokens(
    tokens: &PlanningCenterAccessTokens,
    configuration: &ApplicationConfiguration,
    conn: &mut PgConnection,
) -> AccountTokens
{
    let crypt = new_magic_crypt!(configuration.encryption_key.clone(), 256);
    let encrypted_access_token = crypt.encrypt_str_to_base64(tokens.access_token.clone());
    let encrypted_refresh_token = crypt.encrypt_str_to_base64(tokens.refresh_token.clone());
    to_database(conn, database::PlanningCenterAccessTokens {
        id: tokens.id,
        planning_center_id: tokens.planning_center_id.clone(),
        access_token: encrypted_access_token.clone(),
        refresh_token: encrypted_refresh_token.clone(),
        expires_at: tokens.expires_at
    });

    AccountTokens {
        access_token: encrypted_access_token,
        refresh_token: encrypted_refresh_token,
        expires_at: tokens.expires_at
    }
}