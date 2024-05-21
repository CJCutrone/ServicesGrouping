use serde::Deserialize;

use crate::planning_center::{Meta, PlanningCenterError, Type, TypeVec};

pub async fn team_positions(
    team_id: String,
    team_position_id: String,
    token: String
) -> Result<TeamPositionsAssignmentResponse, PlanningCenterError> {
    let endpoint = format!("https://api.planningcenteronline.com/services/v2/teams/{team_id}/team_positions/{team_position_id}/person_team_position_assignments");
    let response = reqwest::Client::new()
        .get(endpoint)
        .bearer_auth(token)
        .send()
        .await
        .expect("Unable to send request to Planning Center API (endpoint /teams/v2/{{team_id}}/team_positions/{{team_position_id}}/person_team_position_assignments)")
        ;

    let data = response.text().await.expect("Expected response body to have data");

    let response: Response = serde_json::from_str(&data).expect("Failed to parse response body");

    match response {
        Response::Success(success) => Ok(*success),
        Response::Error(error) => Err(error)
    }
}

#[derive(Deserialize)]
enum Response
{
    Success(Box<TeamPositionsAssignmentResponse>),
    Error(PlanningCenterError)
}

#[derive(Deserialize)]
pub struct TeamPositionsAssignmentResponse
{
    pub links: TeamPositionAssignmentLinks,
    pub data: Vec<TeamPositionsAssignmentData>,
    pub included: Vec<String>,
    pub meta: Meta
}

#[derive(Deserialize)]
pub struct TeamPositionsAssignmentData
{
    #[serde(rename = "type")]
    pub _type: String,
    pub id: String,
    pub attributes: TeamPositionAssignmentAttributes,
    pub relationships: TeamPositionAssignmentRelationships,
    pub links: TeamPositionAssignmentDataLinks
}

#[derive(Deserialize)]
pub struct TeamPositionAssignmentAttributes
{
    pub created_at: Option<String>,
    pub preferred_weeks: Vec<String>,
    pub schedule_preference: Option<String>,
    pub updated_at: Option<String>
}

#[derive(Deserialize)]
pub struct TeamPositionAssignmentRelationships
{
    pub person: Type,
    pub team_position: Type,
    pub time_preference_options: TypeVec
}

#[derive(Deserialize)]
pub struct TeamPositionAssignmentLinks
{
    #[serde(rename = "self")]
    pub _self: String
}

#[derive(Deserialize)]
pub struct TeamPositionAssignmentDataLinks
{
    #[serde(rename = "self")]
    pub _self: String
}