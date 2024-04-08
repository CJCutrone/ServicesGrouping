use crate::actions::data::load::user;
use crate::model::database::GroupAssignment;

pub fn from(path: &str) -> Vec<GroupAssignment> {
    let extension = path.split('.').last().unwrap();
    match extension {
        "xlsx" => user::from_excel(path).unwrap().iter().flat_map(GroupAssignment::from_excel).collect(),
        "json" => user::from_json(path).iter().flat_map(GroupAssignment::from_json).collect(),
        _ => panic!("Unsupported file type")
    }
}