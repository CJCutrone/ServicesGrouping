use diesel::prelude::*;
use uuid::Uuid;

use crate::model::{excel, json};

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub planning_center_id: i32,
    pub first_name: String,
    pub last_name: String
}

impl User {
    pub fn from_excel(excel_user: &excel::User) -> User {
        let f_name = excel_user.first_name.clone();
        let l_name = excel_user.last_name.clone();

        //generate a new unique uuid based upon the first and last name
        let id = Uuid::new_v5(&Uuid::NAMESPACE_OID, &format!("{}{}", f_name, l_name).as_bytes());

        User {
            id,
            planning_center_id: -1,
            first_name: f_name,
            last_name: l_name
        }
    }

    pub fn from_json(json_user: &json::User) -> User {
        let f_name = json_user.first_name.clone();
        let l_name = json_user.last_name.clone();
        let planning_center_id = json_user.planning_center_id.clone().unwrap_or_else(|| -1);

        //generate a new unique uuid based upon the first and last name
        let id = Uuid::new_v5(&Uuid::NAMESPACE_OID, &format!("{}{}", f_name, l_name).as_bytes());

        User {
            id,
            planning_center_id,
            first_name: f_name,
            last_name: l_name
        }
    }
}