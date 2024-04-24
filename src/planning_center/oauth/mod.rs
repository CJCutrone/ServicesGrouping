use serde::{Deserialize, Serialize};

use crate::{oauth::model::Account, ApplicationConfiguration};

use super::PlanningCenterError;

pub async fn token(config: &ApplicationConfiguration, code: String) -> Result<Account, PlanningCenterError> {
    let response = reqwest::Client::new()
        .post("https://api.planningcenteronline.com/oauth/token")
        .json(& AuthRequest {
            grant_type: "authorization_code".to_string(),
            code: code.clone(),
            client_id: config.planning_center_id.clone(),
            client_secret: config.planning_center_secret.clone(),
            redirect_uri: format!("{}/{}", config.domain, "oauth/callback").to_string()
        })
        .send()
        .await
        .expect("Unable to send request to Planning Center API (endpoint /oauth/token)")
        ;

    let response: TokenResponse = response.json().await.unwrap();

    match response {
        TokenResponse::Success(success) => {
            Ok(Account {
                access_token: success.access_token,
                refresh_token: success.refresh_token
            })
        },
        TokenResponse::Error(error) => {
            Err(error)
        }
    }
}

pub async fn refresh_token(config: &ApplicationConfiguration, refresh: String) -> Result<Account, PlanningCenterError> {
    let response = reqwest::Client::new()
        .post("https://api.planningcenteronline.com/oauth/token")
        .json(& RefreshRequest {
            grant_type: "refresh_token".to_string(),
            refresh_token: refresh.clone(),
            client_id: config.planning_center_id.clone(),
            client_secret: config.planning_center_secret.clone(),
        })
        .send()
        .await
        .expect("Unable to send request to Planning Center API (endpoint /oauth/token)")
        ;

    let response: TokenResponse = response.json().await.unwrap();

    match response {
        TokenResponse::Success(success) => {
            Ok(Account {
                access_token: success.access_token,
                refresh_token: success.refresh_token
            })
        },
        TokenResponse::Error(error) => {
            Err(error)
        }
    }
}

#[derive(Serialize)]
struct AuthRequest {
    grant_type: String,
    code: String,
    client_id: String,
    client_secret: String,
    redirect_uri: String
}

#[derive(Serialize)]
struct RefreshRequest {
    grant_type: String,
    refresh_token: String,
    client_id: String,
    client_secret: String
}

#[derive(Debug, Serialize, Deserialize)]
struct AuthorizationToken
{
    access_token: String,
    token_type: String,
    expires_in: i64,
    refresh_token: String,
    scope: Option<String>,
    created_at: i64
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum TokenResponse {
    Success(AuthorizationToken),
    Error(PlanningCenterError)
}