use serde::Deserialize;

#[derive(Deserialize)]
pub struct GenerateAssignmentRequests
{
    pub dates: Vec<String>
}