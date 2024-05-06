use serde::Deserialize;

use crate::planning_center::PlanningCenterError;

pub async fn groups(
    group_id: String,
    token: String
) -> Result<GroupResponse, PlanningCenterError> {
    let endpoint = format!("https://api.planningcenteronline.com/groups/v2/groups/{}", group_id);
    let response = reqwest::Client::new()
        .get(endpoint)
        .bearer_auth(token)
        .send()
        .await
        .expect("Unable to send request to Planning Center API (endpoint /group/v2/groups/{{GROUP_ID}})")
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
enum Response {
    Success(Box<GroupResponse>),
    Error(PlanningCenterError)
}

#[derive(Deserialize)]
pub struct GroupResponse
{
    pub data: Group,
    pub included: Vec<String>,
    pub meta: Meta
}

#[derive(Deserialize)]
pub struct Group 
{
    pub id: String,
    #[serde(rename = "type")]
    pub group_type: String,
    pub attributes: Attributes,
    pub relationships: Relationships,
    pub links: Links
}

#[derive(Deserialize)]
pub struct Attributes
{
    pub archived_at: Option<String>,
    pub contact_email: Option<String>,
    pub created_at: String,
    pub description: Option<String>,
    pub events_visibility: String,
    pub header_image: HeaderImage,
    pub location_type_preference: String,
    pub memberships_count: i32,
    pub name: String,
    pub public_church_center_web_url: Option<String>,
    pub schedule: Option<String>,
    pub virtual_location_url: Option<String>
}

#[derive(Deserialize)]
pub struct HeaderImage
{
    pub thumbnail: Option<String>,
    pub medium: Option<String>,
    pub original: Option<String>
}

#[derive(Deserialize)]
pub struct Relationships
{
    pub group_type: GroupType,
    pub location: Location
}

#[derive(Deserialize)]
pub struct GroupType
{
    pub id: String,

    #[serde(rename = "type")]
    pub group_type: String
}

#[derive(Deserialize)]
pub struct Location {
    //TODO: find out struct for Location
}

#[derive(Deserialize)]
pub struct Links
{
    pub enrollment: String,
    pub events: Option<String>,
    pub group_type: Option<String>,
    pub location: Option<String>,
    pub memberships: Option<String>,
    pub people: Option<String>,
    pub resources: Option<String>,
    pub tags: Option<String>,
    #[serde(rename = "self")]
    pub self_: Option<String>,
    pub html: Option<String>
}

#[derive(Deserialize)]
pub struct Meta
{
    pub onboarding: bool,
    pub can_include: Vec<String>,
    pub parent: MetaParent
}

#[derive(Deserialize)]
pub struct MetaParent
{
    pub id: String,

    #[serde(rename = "type")]
    pub parent_type: String
}