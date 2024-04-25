use chrono::NaiveDateTime;
use serde::Deserialize;

use crate::planning_center::{Meta, PlanningCenterError};

pub async fn me(token: String) -> Result<PersonResponse, PlanningCenterError> {
    println!("{:?}", token);

    let response = reqwest::Client::new()
        .get("https://api.planningcenteronline.com/people/v2/me")
        .bearer_auth(token)
        .send()
        .await
        .expect("Unable to send request to Planning Center API (endpoint /people/v2/me)")
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
    Success(Box<PersonResponse>),
    Error(PlanningCenterError)
}

#[derive(Deserialize)]
pub struct PersonResponse
{
    pub data: Person,
    pub included: Vec<String>,
    pub meta: Meta
}

#[derive(Deserialize)]
pub struct Person
{
    pub id: String,
    #[serde(rename = "type")]
    pub me_type: String,
    pub attributes: Attributes,
    pub relationships: Relationships,
    pub links: Links
}

#[derive(Deserialize)]
pub struct Attributes
{
    pub accounting_administrator: bool,
    pub anniversary: Option<String>,
    pub avatar: String,
    pub birthdate: Option<String>,
    pub can_create_forms: bool,
    pub can_email_lists: bool,
    pub child: bool,
    pub created_at: String,
    pub demographic_avatar_url: String,
    pub directory_status: String,
    pub first_name: String,
    pub gender: Option<String>,
    pub given_name: Option<String>,
    pub grade: Option<String>,
    pub graduation_year: Option<String>,
    pub inactivated_at: Option<NaiveDateTime>,
    pub last_name: String,
    pub medical_notes: Option<String>,
    pub membership: Option<String>,
    pub middle_name: Option<String>,
    pub name: String,
    pub nickname: Option<String>,
    pub passed_background_check: bool,
    pub people_permissions: String,
    pub remote_id: Option<String>,
    pub school_type: Option<String>,
    pub site_administrator: bool,
    pub status: String,
    pub updated_at: String
}

#[derive(Deserialize)]
pub struct Relationships
{
    pub primary_campus: PrimaryCampus,
    pub gender: Gender
}

#[derive(Deserialize)]
pub struct PrimaryCampus
{
    pub data: Option<CampusData>
}

#[derive(Deserialize)]
pub struct CampusData
{
    pub id: String,
    #[serde(rename = "type")]
    pub campus_type: String
}

#[derive(Deserialize)]
pub struct Gender
{
    pub data: Option<GenderData>
}

#[derive(Deserialize)]
pub struct GenderData
{
    pub id: String,
    #[serde(rename = "type")]
    pub gender_type: String
}

#[derive(Deserialize)]
pub struct Links
{
    #[serde(rename = "")]
    pub self_: String,
    pub addresses: Option<String>,
    pub apps: Option<String>,
    pub connected_people: Option<String>,
    pub emails: Option<String>,
    pub field_data: Option<String>,
    pub household_memberships: Option<String>,
    pub households: Option<String>,
    pub inactive_reason: Option<String>,
    pub marital_status: Option<String>,
    pub message_groups: Option<String>,
    pub messages: Option<String>,
    pub name_prefix: Option<String>,
    pub name_suffix: Option<String>,
    pub notes: Option<String>,
    pub organization: Option<String>,
    pub person_apps: Option<String>,
    pub phone_numbers: Option<String>,
    pub platform_notifications: Option<String>,
    pub primary_campus: Option<String>,
    pub school: Option<String>,
    pub social_profiles: Option<String>,
    pub workflow_cards: Option<String>,
    pub workflow_shares: Option<String>,
    pub html: Option<String>
}