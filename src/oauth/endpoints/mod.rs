use actix_session::Session;
use actix_web::{get, web::{Data, Query}, HttpResponse, Responder};
use diesel::{r2d2::{ConnectionManager, Pool}, PgConnection};
use serde::Deserialize;
use uuid::Uuid;

use crate::{actions, model::database, oauth::model::Account, planning_center, ApplicationConfiguration};

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

    let mut conn = pool.get_ref().get().expect("Error getting connection");
    actions::data::save::account::to_database(&mut conn, database::Account {
        id: Uuid::new_v5(&Uuid::NAMESPACE_OID, me_response.data.id.as_bytes()),
        planning_center_id: me_response.data.id.clone(),
        access_token: response.access_token.clone(),
        refresh_token: response.refresh_token.clone()
    });

    let _ = session.insert("account", response);
    HttpResponse::Ok().finish()
}

#[get("/oauth/refresh")]
async fn refresh_token(
    session: Session,
    configuration: Data<ApplicationConfiguration>
) -> impl Responder {
    let configuration = configuration.get_ref();
    let account = session.get::<Account>("account").expect("Unable to retrieve account from session").unwrap();

    let response = planning_center::oauth::refresh_token(configuration, account.refresh_token.clone()).await.expect("Unable to verify access token");

    let _ = session.insert("account", response);
    HttpResponse::Ok().finish()
}

#[get("/oauth/me")]
async fn me(
    session: Session
) -> impl Responder {    
    match session.get::<Account>("account") {
        Ok(Some(account)) => {
            HttpResponse::Ok().json(account)
        },
        _ => {
            HttpResponse::Unauthorized().finish()
        }
    }
}

#[derive(Deserialize)]
struct OAuthCallback {
    code: String
}