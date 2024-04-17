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

    match group.len() {
        0 => GetGroupResult::NotFound,
        1 => GetGroupResult::Success(group[0].clone()),
        _ => GetGroupResult::MoreThanOneFound
    }
}

pub enum GetGroupResult
{
    Success(Group),
    NotFound,
    MoreThanOneFound,
    UnknownDatabaseError(Error)
}