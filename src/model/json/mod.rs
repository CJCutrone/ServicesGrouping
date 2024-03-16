use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
    pub positions: i32
}

impl User {
    pub fn uuid(&self) -> Uuid {
        Uuid::new_v5(&Uuid::NAMESPACE_OID, &format!("{}{}", self.first_name, self.last_name).as_bytes())
    }
}

impl Group {
    pub fn uuid(&self) -> Uuid {
        Uuid::new_v5(&Uuid::NAMESPACE_OID, &self.name.as_bytes())
    }
}