use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;
use uuid::Uuid;

#[derive(Queryable, Selectable, Insertable, Serialize, Clone, Debug)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub planning_center_id: String,
    pub first_name: String,
    pub last_name: String,
    pub is_deleted: bool
}

#[derive(Queryable, Selectable, Insertable, Serialize, Clone, Debug)]
#[diesel(table_name = crate::schema::groups)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Group {
    pub id: Uuid,
    pub planning_center_id: String,
    pub name: String,
    pub positions: i32,
    pub is_deleted: bool
}

#[derive(Queryable, Selectable, Insertable, Associations, Serialize, Debug)]
#[diesel(table_name = crate::schema::group_assignments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Group))]
pub struct GroupAssignment {
    pub id: Uuid,
    pub user_id: Uuid,
    pub group_id: Uuid,
    pub tickets: i32
}

#[derive(Queryable, Selectable, Insertable, Associations, Serialize, Debug)]
#[diesel(table_name = crate::schema::service_dates)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(GroupAssignment))]
pub struct ServiceDate {
    pub id: Uuid,
    pub group_assignment_id: Uuid,
    pub tickets_consumed: i32,
    pub for_date: NaiveDateTime,
}