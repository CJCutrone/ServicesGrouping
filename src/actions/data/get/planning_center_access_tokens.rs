use diesel::prelude::*;
use diesel::{result::Error, PgConnection, SelectableHelper};
use log::error;
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use uuid::Uuid;

use crate::model::database::PlanningCenterAccessTokens;
use crate::schema::planning_center_access_tokens::dsl::planning_center_access_tokens;
use crate::schema::planning_center_access_tokens::id;
use crate::ApplicationConfiguration;

pub fn by_id(
    conn: &mut PgConnection, 
    account_id: Uuid
) -> GetPlanningCenterAccessTokensResult {
    let result = planning_center_access_tokens
        .filter(id.eq(account_id))
        .select(PlanningCenterAccessTokens::as_select())
        .load(conn)
        ;

    if let Err(e) = result {
        return GetPlanningCenterAccessTokensResult::UnknownDatabaseError(e);
    }

    let account = result.unwrap();

    if account.is_empty() {
        return GetPlanningCenterAccessTokensResult::NotFound;
    }

    if account.len() > 1 {
        return GetPlanningCenterAccessTokensResult::MoreThanOneFound;
    }
    
    GetPlanningCenterAccessTokensResult::Success(account[0].clone())
}

pub fn get_decrypted_tokens_for_account(
    account_id: Uuid,
    configuration: &ApplicationConfiguration,
    conn: &mut PgConnection
) -> DecryptAccountTokensResult
{
    match by_id(conn, account_id) {
        GetPlanningCenterAccessTokensResult::UnknownDatabaseError(e) => {
            error!("{:?}", e);
            DecryptAccountTokensResult::UnknownDatabaseError(e)
        },
        GetPlanningCenterAccessTokensResult::MoreThanOneFound => {
            error!("More than one account found for ID: {:?}", account_id);
            DecryptAccountTokensResult::MoreThanOneAccountFound
        },
        GetPlanningCenterAccessTokensResult::NotFound => {
            error!("Account not found with ID: {:?}", account_id);
            DecryptAccountTokensResult::NoAccountFound
        },
        GetPlanningCenterAccessTokensResult::Success(account) => {
            let crypt = new_magic_crypt!(configuration.encryption_key.clone(), 256);
            let decrypted_access_token = crypt.decrypt_base64_to_string(account.access_token);
            let decrypted_refresh_token = crypt.decrypt_base64_to_string(account.refresh_token);

            if decrypted_access_token.is_err() || decrypted_refresh_token.is_err() {
                return DecryptAccountTokensResult::UnableToDecryptTokens;
            }

            let decrypted_access_token = decrypted_access_token.unwrap();
            let decrypted_refresh_token = decrypted_refresh_token.unwrap();
            
            DecryptAccountTokensResult::Success(PlanningCenterAccessTokens {
                id: account.id,
                planning_center_id: account.planning_center_id,
                access_token: decrypted_access_token,
                refresh_token: decrypted_refresh_token,
                expires_at: account.expires_at
            })
        }
    }
}

pub enum GetPlanningCenterAccessTokensResult
{
    Success(PlanningCenterAccessTokens),
    NotFound,
    MoreThanOneFound,
    UnknownDatabaseError(Error)
}

pub enum DecryptAccountTokensResult {
    UnknownDatabaseError(Error),
    MoreThanOneAccountFound,
    NoAccountFound,
    UnableToDecryptTokens,
    Success(PlanningCenterAccessTokens)
}