use crate::actions::data::load::user;
use crate::model::database::GroupAssignment;

pub fn from(path: &str) -> Vec<GroupAssignment> {
    let extension = path.split('.').last().unwrap();
    match extension {
        "xlsx" => user::from_excel(path).unwrap().iter().map(|item| GroupAssignment::from_excel(item)).flatten().collect(),
        "json" => user::from_json(path).iter().map(|item| GroupAssignment::from_json(item)).flatten().collect(),
        _ => panic!("Unsupported file type")
    }
}