use serde::{Deserialize, Serialize};

use crate::api::webhooks::WebhookRequest;

#[derive(Serialize, Deserialize, Debug)]
pub struct Group
{
    pub data: GroupData
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GroupData
{
    pub id: String,
    pub attributes: Option<GroupAttributes>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GroupAttributes
{
    pub name: String
}

impl Group
{
    pub fn from(request: WebhookRequest<String>) -> Self
    {
        let payload = &request.data[0].attributes.payload;
        let payload: Group = serde_json::from_str(payload).unwrap();
        payload
    }
}