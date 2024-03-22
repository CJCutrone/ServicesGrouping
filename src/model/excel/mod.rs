use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub first_name: String,
    pub last_name: String,
    #[serde(deserialize_with = "deserialize_as_vec")]
    pub groups: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Group {
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

fn deserialize_as_vec<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
    where
        D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(s.split(',').map(|s| s.trim().to_string()).collect())
}