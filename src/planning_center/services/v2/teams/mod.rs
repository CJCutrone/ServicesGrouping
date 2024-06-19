use serde::Deserialize;

use crate::planning_center::{Meta, PlanningCenterError, Type, TypeVec};

pub mod team_positions;

pub async fn team_positions(
    team_id: &String,
    token: &String
) -> Result<TeamPositionsResponse, PlanningCenterError> {
    let endpoint = format!("https://api.planningcenteronline.com/services/v2/teams/{team_id}/team_positions");
    let response = reqwest::Client::new()
        .get(endpoint)
        .bearer_auth(token)
        .send()
        .await
        .expect("Unable to send request to Planning Center API (endpoint /teams/v2/{{team_id}}/team_positions)")
        ;

    let data = response.text().await.expect("Expected response body to have data");

    println!("{:?}", data);

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
    Success(Box<TeamPositionsResponse>),
    Error(PlanningCenterError)
}

#[derive(Deserialize)]
pub struct TeamPositionsResponse
{
    pub links: TeamPositionsLinks,
    pub data: Vec<TeamPositionData>,
    pub included: Vec<String>,
    pub meta: Meta
}

#[derive(Deserialize)]
pub struct TeamPositionsLinks
{
    #[serde(rename = "self")]
    pub _self: String
}

#[derive(Deserialize)]
pub struct TeamPositionData
{
    #[serde(rename = "type")]
    pub _type: String,
    pub id: String,
    pub attributes: TeamPositionAttributes,
    pub relationships: TeamPositionRelationships,
    pub links: TeamPositionLinks
}

#[derive(Deserialize)]
pub struct TeamPositionLinks
{
    #[serde(rename = "self")]
    pub _self: String
}

#[derive(Deserialize)]
pub struct TeamPositionAttributes
{
    pub name: String,
    pub negative_tag_groups: Option<Vec<String>>,
    pub sequence: Option<String>,
    pub tag_groups: Option<Vec<String>>,
    pub tags: Option<Vec<String>>
}

#[derive(Deserialize)]
pub struct TeamPositionRelationships
{
    pub team: Type,
    pub attachment_types: TypeVec,
    pub tags: TypeVec
}