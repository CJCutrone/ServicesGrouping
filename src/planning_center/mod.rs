use serde::{Deserialize, Serialize};

pub mod oauth;
pub mod people;

#[derive(Debug, Deserialize, Serialize)]
pub struct PlanningCenterError {
    pub error: String,
    pub error_description: String
}

#[derive(Deserialize)]
pub struct Meta
{
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