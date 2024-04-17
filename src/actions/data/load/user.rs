use calamine::{open_workbook, RangeDeserializerBuilder, Reader, Xlsx};
use serde::{Deserialize, Serialize};
use crate::model::{excel, json};
use crate::model::database::User;

pub fn from(path: &str) -> Vec<User> {
    let extension = path.split('.').last().unwrap();
    match extension {
        "xlsx" => from_excel(path).unwrap().iter().map(User::from_excel).collect(),
        "json" => from_json(path).iter().map(User::from_json).collect(),
        _ => panic!("Unsupported file type")
    }
}

pub fn from_excel(path: &str) -> Result<Vec<excel::User>, calamine::Error> {
    let mut workbook: Xlsx<_> = open_workbook(path).unwrap();

    let range = workbook
        .worksheet_range("Users")
        .map_err(|_| calamine::Error::Msg("Cannot find sheet 'Users'"))?;

    let iter_records = RangeDeserializerBuilder::with_headers(&["first_name", "last_name", "groups"])
        .from_range(&range)?;

    Ok(iter_records.map(|r| r.unwrap()).collect())
}

//same as from_excel, but for json
pub fn from_json(path: &str) -> Vec<json::User> {
    let content = std::fs::read_to_string(path).unwrap();
    let data = serde_json::from_str::<JsonData>(&content).expect("JSON was not well formatted");
    data.users
}

#[derive(Debug, Serialize, Deserialize)]
struct JsonData
{
    users: Vec<json::User>
}