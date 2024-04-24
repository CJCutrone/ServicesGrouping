use actix_session::Session;
use actix_web::{get, web::{Data, Query}, HttpResponse, Responder};
use serde::Deserialize;

use crate::{oauth::model::Account, planning_center, ApplicationConfiguration};

#[get("/oauth/callback")]
async fn callback(
    session: Session,
    params: Query<OAuthCallback>,
    configuration: Data<ApplicationConfiguration>
) -> impl Responder {
    let configuration = configuration.get_ref();
    let response = planning_center::oauth::token(configuration, params.code.clone()).await.expect("Unable to verify access token");

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