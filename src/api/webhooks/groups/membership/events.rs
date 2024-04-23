use serde::{Deserialize, Serialize};

use crate::api::webhooks::WebhookRequest;

#[derive(Serialize, Deserialize, Debug)]
pub struct MembershipPayload {
    pub data: MembershipData
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MembershipData{
    pub id: String,
    pub relationships: MembershipRelationships
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MembershipRelationships{
    pub group: Group,
    pub person: Person
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Group{
    pub data: RelationshipData
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Person{
    pub data: RelationshipData
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RelationshipData{
    pub id: String,
    #[serde(rename = "type")]
    pub relationship_type: String
}

impl MembershipPayload
{
    pub fn from(request: WebhookRequest<String>) -> Self
    {
        let payload = &request.data[0].attributes.payload;
        let payload: MembershipPayload = serde_json::from_str(payload).unwrap();
        payload
    }

}