use serde::Deserialize;

use crate::planning_center::{Meta, PlanningCenterError, Type};

pub mod teams;

pub async fn teams(
    team_id: String,
    token: String
) -> Result<TeamResponse, PlanningCenterError> {
    let endpoint = format!("https://api.planningcenteronline.com/services/v2/teams/{team_id}");
    let response = reqwest::Client::new()
        .get(endpoint)
        .bearer_auth(token)
        .send()
        .await
        .expect("Unable to send request to Planning Center API (endpoint /teams/v2/{{team_id}})")
        ;

    let data = response.text().await.expect("Expected response body to have data");

    let response: Response = serde_json::from_str(&data).expect("Failed to parse response body");

    match response {
        Response::Success(success) => Ok(*success),
        Response::Error(error) => Err(error)
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
enum Response
{
    Success(Box<TeamResponse>),
    Error(PlanningCenterError)
}

#[derive(Deserialize)]
pub struct TeamResponse
{
    pub data: TeamData,
    pub included: Vec<String>,
    pub meta: Meta
}

#[derive(Deserialize)]
pub struct TeamData
{
    #[serde(rename = "type")]
    pub _type: String,
    pub id: String,
    pub attributes: TeamAttributes,
    pub relationships: TeamRelationships,
    pub links: TeamLinks
}

#[derive(Deserialize)]
pub struct TeamAttributes
{
    pub archived_at: Option<String>,
    pub assigned_directly: bool,
    pub created_at: Option<String>,
    pub default_prepare_notifications: bool,
    pub default_status: Option<String>,
    pub last_plan_from: Option<String>,
    pub name: Option<String>,
    pub rehearsal_team: bool,
    pub schedule_to: Option<String>,
    pub secure_team: bool,
    pub sequence: Option<String>,
    pub stage_color: Option<String>,
    pub stage_variant: Option<String>,
    pub updated_at: Option<String>,
    pub viewers_see: i32
}

#[derive(Deserialize)]
pub struct TeamRelationships
{
    pub service_type: Type,
    pub default_responds_to: Type
}

#[derive(Deserialize)]
pub struct TeamLinks
{
    pub people: Option<String>,
    pub person_team_position_assignments: Option<String>,
    pub plan_people: Option<String>,
    pub service_type: Option<String>,
    pub team_leaders: Option<String>,
    pub team_positions: Option<String>,
    pub _self: Option<String>
}