use diesel::prelude::*;
use uuid::Uuid;

use crate::model::{excel, json};

#[derive(Queryable, Selectable, Insertable, Debug)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub planning_center_id: i32,
    pub first_name: String,
    pub last_name: String
}

#[derive(Queryable, Selectable, Insertable, Debug)]
#[diesel(table_name = crate::schema::groups)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Group {
    pub id: Uuid,
    pub planning_center_id: i32,
    pub name: String,
    pub positions: i32
}

#[derive(Queryable, Selectable, Insertable, Associations, Debug)]
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

impl User {
    pub fn from_excel(excel_user: &excel::User) -> User {
        let f_name = excel_user.first_name.clone();
        let l_name = excel_user.last_name.clone();
        let id = excel_user.uuid();

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
        let id = json_user.uuid();

        User {
            id,
            planning_center_id,
            first_name: f_name,
            last_name: l_name
        }
    }
}

impl GroupAssignment {
    pub fn from_excel(excel_user: &excel::User) -> Vec<GroupAssignment> {
        let user_id = excel_user.uuid();
        return excel_user.groups.iter().map(|g| group_assignment(user_id, g)).collect();
    }

    pub fn from_json(json_user: &json::User) -> Vec<GroupAssignment> {
        let user_id = json_user.uuid();
        return json_user.groups.iter().map(|g| group_assignment(user_id, g)).collect();
    }
}

impl Group {
    pub fn from_excel(group: &excel::Group) -> Group {
        let id = group.uuid();

        Group {
            id,
            planning_center_id: -1,
            name: group.name.clone(),
            positions: group.positions.clone()
        }
    }

    pub fn from_json(group: &json::Group) -> Group {
        let id = group.uuid();

        Group {
            id,
            planning_center_id: group.planning_center_id.clone().unwrap_or_else(|| -1),
            name: group.name.clone(),
            positions: group.positions.clone()
        }
    }
}

fn group_assignment(user_id: Uuid, group: &String) -> GroupAssignment {
    let group_id =Uuid::new_v5(&Uuid::NAMESPACE_OID, group.as_bytes());
    GroupAssignment {
        id: Uuid::new_v5(&Uuid::NAMESPACE_OID, &format!("{}{}", user_id, group_id).as_bytes()),
        user_id,
        group_id,
        tickets: 0
    }
}