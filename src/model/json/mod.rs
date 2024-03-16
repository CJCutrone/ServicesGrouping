use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub planning_center_id: Option<i32>,
    pub first_name: String,
    pub last_name: String,
    pub groups: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Group {
    pub planning_center_id: Option<i32>,
    pub name: String,
    pub required: i8
}