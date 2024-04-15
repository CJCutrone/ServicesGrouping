use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod groups;


#[derive(Serialize, Deserialize, Debug)]
pub struct WebhookRequest<T> {
    pub data: Vec<WebhookData<T>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WebhookData<T> {
    pub id: Uuid,
    #[serde(rename = "type")]
    pub webhook_type: String,
    pub attributes: WebhookAttributes<T>,
    pub relationships: WebhookRelationships
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WebhookAttributes<T>
{
    pub name: String,
    pub attempt: i8,
    pub payload: T
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WebhookRelationships
{
    pub organization: WebhookOrganization
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WebhookOrganization
{
    pub data: WebhookOrganizationData
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WebhookOrganizationData
{
    #[serde(rename = "type")]
    pub organization_type: String,
    pub id: String
}