use serde::{Deserialize, Serialize};

pub mod oauth;

#[derive(Debug, Deserialize, Serialize)]
pub struct PlanningCenterError {
    pub error: String,
    pub error_description: String
}