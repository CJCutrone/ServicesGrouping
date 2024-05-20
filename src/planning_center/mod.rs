use serde::{Deserialize, Serialize};

pub mod group;
pub mod oauth;
pub mod people;
pub mod services;

#[derive(Debug, Deserialize, Serialize)]
pub struct PlanningCenterError {
    pub error: String,
    pub error_description: String
}

#[derive(Deserialize)]
pub struct Meta
{
    pub total_count: Option<i32>,
    pub count: Option<i32>,
    pub can_include: Vec<String>,
    pub can_order_by: Option<Vec<String>>,
    pub parent: Data
}

#[derive(Deserialize)]
pub struct Type
{
    pub data: Data,
}

#[derive(Deserialize)]
pub struct TypeVec
{
    pub data: Vec<Data>,
}

#[derive(Deserialize)]
pub struct Data
{
    pub id: String,

    #[serde(rename = "type")]
    pub _type: String
}