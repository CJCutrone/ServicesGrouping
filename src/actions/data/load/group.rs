use calamine::{open_workbook, RangeDeserializerBuilder, Reader, Xlsx};
use crate::model::{excel, json};

pub fn from_excel(path: &str) -> Result<Vec<excel::Group>, calamine::Error> {
    let mut workbook: Xlsx<_> = open_workbook(path).unwrap();

    let range = workbook
        .worksheet_range("Groups")
        .map_err(|_| calamine::Error::Msg("Cannot find sheet 'Groups'"))?;

    let iter_records = RangeDeserializerBuilder::with_headers(&["name", "required"])
        .from_range(&range)?;

    Ok(iter_records.map(|r| r.unwrap()).collect())
}

pub fn from_json(path: &str) -> Vec<json::Group> {
    let content = std::fs::read_to_string(path).unwrap();
    let groups = serde_json::from_str::<Vec<json::Group>>(&content).expect("JSON was not well formatted");
    return groups;
}