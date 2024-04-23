use std::env;

use actix_session::Session;
use actix_web::{get, web::Query, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::oauth::model::Account;

#[get("/oauth/callback")]
async fn callback(
    session: Session,
    params: Query<OAuthCallback>
) -> impl Responder {    
    let domain = env::var("SERVER_DOMAIN").expect("SERVER_DOMAIN must be set");
    let center_id = env::var("PLANNING_CENTER_ID").expect("PLANNING_CENTER_ID must be set");
    let secret = env::var("PLANNING_CENTER_SECRET").expect("PLANNING_CENTER_SECRET must be set");

    let response = reqwest::Client::new()
        .post("https://api.planningcenteronline.com/oauth/token")
        .json(& AuthRequest {
            grant_type: "authorization_code".to_string(),
            code: params.code.clone(),
            client_id: center_id,
            client_secret: secret,
            redirect_uri: format!("{}/{}", domain, "oauth/callback").to_string()
        })
        .send()
        .await
        ;

    let response = response.unwrap();
    let response: Account = response.json().await.unwrap();

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


#[derive(Serialize)]
pub struct AuthRequest {
    pub grant_type: String,
    pub code: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String
}