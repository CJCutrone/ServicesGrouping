use diesel::prelude::*;
use diesel::{r2d2::{ConnectionManager, Pool}, result::Error, PgConnection, SelectableHelper};

use crate::model::database::Group;
use crate::schema::groups::dsl::groups;
use crate::schema::groups::planning_center_id;

pub fn by_planning_center_id(
    pool: Pool<ConnectionManager<PgConnection>>, 
    by_planning_center_id: &str
) -> GetGroupResult {
    let mut connection = pool.get().expect("Error getting connection");
    let result = groups
        .filter(planning_center_id.eq(by_planning_center_id))
        .select(Group::as_select())
        .load(&mut connection)
        ;

    if let Err(e) = result {
        return GetGroupResult::UnknownDatabaseError(e);
    }

    let group = result.unwrap();

    if group.is_empty() {
        return GetGroupResult::NotFound;
    }

    if group.len() > 1 {
        return GetGroupResult::MoreThanOneFound;
    }

    let group = &group[0];
    if group.is_deleted { 
        return GetGroupResult::GroupDeleted 
    }
    
    GetGroupResult::Success(group.clone())
}

pub enum GetGroupResult
{
    Success(Group),
    NotFound,
    MoreThanOneFound,
    GroupDeleted,
    UnknownDatabaseError(Error)
}