use actix_session::Session;
use actix_web::{get, web::{Data, Query}, HttpResponse, Responder};
use diesel::{r2d2::{ConnectionManager, Pool}, PgConnection};
use log::error;
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use serde::Deserialize;
use uuid::Uuid;

use crate::{actions::{self, data::get::account::GetAccountResult}, model::database::{self, Account}, oauth::model::AccountTokens, planning_center, ApplicationConfiguration};

#[get("/oauth/callback")]
async fn callback(
    session: Session,
    params: Query<OAuthCallback>,
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
    configuration: Data<ApplicationConfiguration>
) -> impl Responder {
    let configuration = configuration.get_ref();

    let response = planning_center::oauth::token(configuration, params.code.clone()).await.expect("Unable to verify access token");
    let me_response = planning_center::people::v2::me(response.access_token.clone()).await.expect("Unable to retrieve account information");

    let user_id = Uuid::new_v5(&Uuid::NAMESPACE_OID, me_response.data.id.as_bytes());

    let crypt = new_magic_crypt!(configuration.encryption_key.clone(), 256);
    let encrypted_access_token = crypt.encrypt_str_to_base64(response.access_token);
    let encrypted_refresh_token = crypt.encrypt_str_to_base64(response.refresh_token);

    let mut conn = pool.get_ref().get().expect("Error getting connection");

    let account = Account {
        id: user_id,
        planning_center_id: me_response.data.id.clone(),
        access_token: encrypted_access_token,
        refresh_token: encrypted_refresh_token
    };

    actions::data::save::account::to_database(&mut conn, account.clone());

    let _ = session.insert("account", account);
    HttpResponse::Ok().finish()
}

#[get("/oauth/refresh")]
async fn refresh_token(
    account: Account,
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
    configuration: Data<ApplicationConfiguration>
) -> impl Responder {
    let configuration = configuration.get_ref();
    let mut conn = pool.get_ref().get().expect("Error getting connection");

    //get current access token and refresh token from database
    let account = match actions::data::get::account::by_id(&mut conn, account.id) {
        GetAccountResult::UnknownDatabaseError(e) => {
            error!("{:?}", e);
            Err(())
        },
        GetAccountResult::MoreThanOneFound => {
            error!("More than one account found for ID: {:?}", account.id);
            Err(())
        },
        GetAccountResult::NotFound => {
            error!("Account not found with ID: {:?}", account.id);
            Err(())
        },
        GetAccountResult::Success(account) => Ok(account)
    };

    if account.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    let account = account.unwrap();
    let crypt = new_magic_crypt!(configuration.encryption_key.clone(), 256);
    let decrypted_refresh_token = crypt.decrypt_base64_to_string(account.refresh_token);
    
    if let Err(e) = decrypted_refresh_token {
        error!("{:?}", e);
        return HttpResponse::InternalServerError().finish();
    }

    let decrypted_refresh_token = decrypted_refresh_token.unwrap();

    let response = planning_center::oauth::refresh_token(configuration, decrypted_refresh_token).await;
    
    if let Err(e) = response {
        error!("{:?}", e.error);
        return HttpResponse::InternalServerError().finish();
    }

    let response = response.unwrap();

    let crypt = new_magic_crypt!(configuration.encryption_key.clone(), 256);
    let encrypted_access_token = crypt.encrypt_str_to_base64(response.access_token);
    let encrypted_refresh_token = crypt.encrypt_str_to_base64(response.refresh_token);
    actions::data::save::account::to_database(&mut conn, database::Account {
        id: account.id,
        planning_center_id: account.planning_center_id,
        access_token: encrypted_access_token,
        refresh_token: encrypted_refresh_token
    });
    
    HttpResponse::Ok().finish()
}

//TODO: should either be removed after testing is done, or should be modified to not return auth/refresh tokens
#[get("/oauth/me")]
async fn me(
    account: Account,
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
    configuration: Data<ApplicationConfiguration>
) -> impl Responder {
    let mut conn = pool.get_ref().get().expect("Error getting connection");

    match actions::data::get::account::by_id(&mut conn, account.id) {
        GetAccountResult::UnknownDatabaseError(e) => {
            error!("{:?}", e);
            HttpResponse::InternalServerError().finish()
        },
        GetAccountResult::MoreThanOneFound => {
            error!("More than one account found for ID: {:?}", account.id);
            HttpResponse::InternalServerError().finish()
        },
        GetAccountResult::NotFound => {
            error!("Account not found with ID: {:?}", account.id);
            HttpResponse::Unauthorized().finish()
        },
        GetAccountResult::Success(account) => {
            let crypt = new_magic_crypt!(configuration.get_ref().encryption_key.clone(), 256);
            let decrypted_access_token = crypt.decrypt_base64_to_string(account.access_token);
            let decrypted_refresh_token = crypt.decrypt_base64_to_string(account.refresh_token);

            if decrypted_access_token.is_err() || decrypted_refresh_token.is_err() {
                return HttpResponse::InternalServerError().finish();
            }

            let decrypted_access_token = decrypted_access_token.unwrap();
            let decrypted_refresh_token = decrypted_refresh_token.unwrap();

            HttpResponse::Ok().json(AccountTokens {
                access_token: decrypted_access_token,
                refresh_token: decrypted_refresh_token
            })
        }
    }
}

#[derive(Deserialize)]
struct OAuthCallback {
    code: String
}