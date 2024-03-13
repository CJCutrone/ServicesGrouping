use diesel::prelude::*;
use uuid::Uuid;
use crate::model::excel;

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
    pub fn from(excel_user: &excel::User) -> User {
        User {
            id: Uuid::new_v4(),
            planning_center_id: -1,
            first_name: excel_user.first_name.clone(),
            last_name: excel_user.last_name.clone()
        }
    }
}