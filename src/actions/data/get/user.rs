use diesel::prelude::*;
use diesel::{result::Error, PgConnection, SelectableHelper};

use crate::model::database::User;
use crate::schema::users::dsl::users;
use crate::schema::users::planning_center_id;

pub fn by_planning_center_id(
    conn: &mut PgConnection, 
    by_planning_center_id: &str
) -> GetUserResult {
    let result = users
        .filter(planning_center_id.eq(by_planning_center_id))
        .select(User::as_select())
        .load(conn)
        ;

    if let Err(e) = result {
        return GetUserResult::UnknownDatabaseError(e);
    }

    let user = result.unwrap();

    if user.is_empty() {
        return GetUserResult::NotFound;
    }

    if user.len() > 1 {
        return GetUserResult::MoreThanOneFound;
    }

    let user = &user[0];
    if user.is_deleted { 
        return GetUserResult::UserDeleted 
    }
    
    GetUserResult::Success(user.clone())
}

pub enum GetUserResult
{
    Success(User),
    NotFound,
    MoreThanOneFound,
    UserDeleted,
    UnknownDatabaseError(Error)
}