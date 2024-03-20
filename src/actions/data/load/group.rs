use calamine::{open_workbook, RangeDeserializerBuilder, Reader, Xlsx};
use serde::{Deserialize, Serialize};
use crate::model::{excel, json};

pub fn from(path: &str) -> Vec<crate::model::database::Group> {
    let extension = path.split('.').last().unwrap();
    match extension {
        "xlsx" => from_excel(path).unwrap().iter().map(|item| crate::model::database::Group::from_excel(item)).collect(),
        "json" => from_json(path).iter().map(|item| crate::model::database::Group::from_json(item)).collect(),
        _ => panic!("Unsupported file type")
    }
}

pub fn from_excel(path: &str) -> Result<Vec<excel::Group>, calamine::Error> {
    let mut workbook: Xlsx<_> = open_workbook(path).unwrap();

    let range = workbook
        .worksheet_range("Groups")
        .map_err(|_| calamine::Error::Msg("Cannot find sheet 'Groups'"))?;

    let iter_records = RangeDeserializerBuilder::with_headers(&["name", "positions"])
        .from_range(&range)?;

    Ok(iter_records.map(|r| r.unwrap()).collect())
}

pub fn from_json(path: &str) -> Vec<json::Group> {
    let content = std::fs::read_to_string(path).unwrap();
    let data = serde_json::from_str::<JsonData>(&content).expect("JSON was not well formatted");
    return data.groups;
}

#[derive(Debug, Serialize, Deserialize)]
struct JsonData
{
    groups: Vec<json::Group>
}