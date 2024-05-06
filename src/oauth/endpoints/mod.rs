use actix_session::Session;
use actix_web::{get, web::{Data, Query}, HttpResponse, Responder};
use diesel::{r2d2::{ConnectionManager, Pool}, PgConnection};
use log::error;
use serde::Deserialize;
use uuid::Uuid;

use crate::{actions::data::{get::planning_center_access_tokens::{get_decrypted_tokens_for_account, DecryptAccountTokensResult}, save::planning_center_access_tokens::encrypt_and_store_tokens}, model::database::PlanningCenterAccessTokens, oauth::model::AccountTokens, planning_center, ApplicationConfiguration};

#[get("/oauth/callback")]
async fn callback(
    session: Session,
    params: Query<OAuthCallback>,
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
    configuration: Data<ApplicationConfiguration>
) -> impl Responder {
    let configuration = configuration.get_ref();

    let get_token_response = planning_center::oauth::token(configuration, params.code.clone()).await.expect("Unable to verify access token");
    let get_user_info_response = planning_center::people::v2::me(get_token_response.access_token.clone()).await.expect("Unable to retrieve account information");

    let planning_center_id = get_user_info_response.data.id.clone();
    let user_id = Uuid::new_v5(&Uuid::NAMESPACE_OID, planning_center_id.as_bytes());
    
    let planning_center_access_tokens = PlanningCenterAccessTokens {
        id: user_id,
        planning_center_id: planning_center_id.clone(),
        access_token: get_token_response.access_token,
        refresh_token: get_token_response.refresh_token,
        expires_at: get_token_response.expires_at
    };

    let mut conn = pool.get_ref().get().expect("Error getting connection");
    let encrypted = encrypt_and_store_tokens(&planning_center_access_tokens, configuration, &mut conn);

    let account = PlanningCenterAccessTokens {
        id: user_id,
        planning_center_id,
        access_token: encrypted.access_token,
        refresh_token: encrypted.refresh_token,
        expires_at: encrypted.expires_at
    };

    let _ = session.insert("account", account);
    HttpResponse::Ok().finish()
}

#[get("/oauth/refresh")]
async fn refresh_token(
    account: PlanningCenterAccessTokens,
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
    configuration: Data<ApplicationConfiguration>
) -> impl Responder {
    let configuration = configuration.get_ref();
    let mut conn = pool.get_ref().get().expect("Error getting connection");

    match get_decrypted_tokens_for_account(account.id, configuration, &mut conn) {
        DecryptAccountTokensResult::Success(tokens) => {
            let response = planning_center::oauth::refresh_token(configuration, tokens.refresh_token).await;
            if let Err(e) = response {
                error!("{:?}", e.error);
                return HttpResponse::InternalServerError().finish();
            }
        
            let response = response.unwrap();

            let planning_center_access_tokens = PlanningCenterAccessTokens {
                id: account.id,
                planning_center_id: account.planning_center_id,
                access_token: response.access_token.clone(),
                refresh_token: response.refresh_token.clone(),
                expires_at: response.expires_at
            };

            let tokens = encrypt_and_store_tokens(&planning_center_access_tokens, configuration, &mut conn);

            HttpResponse::Ok().json(AccountTokens {
                access_token: tokens.access_token,
                refresh_token: tokens.refresh_token,
                expires_at: tokens.expires_at
            })
        },
        _ => HttpResponse::InternalServerError().finish()
    }
}

//TODO: should either be removed after testing is done, or should be modified to not return auth/refresh tokens
#[get("/oauth/me")]
async fn me(
    account: PlanningCenterAccessTokens,
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
    configuration: Data<ApplicationConfiguration>
) -> impl Responder {
    let configuration = configuration.get_ref();
    let mut conn = pool.get_ref().get().expect("Error getting connection");

    match get_decrypted_tokens_for_account(account.id, configuration, &mut conn) {
        DecryptAccountTokensResult::Success(tokens) => {
            HttpResponse::Ok().json(tokens)
        },
        _ => HttpResponse::InternalServerError().finish()
    }
}

#[derive(Deserialize)]
struct OAuthCallback {
    code: String
}