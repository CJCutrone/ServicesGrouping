use diesel::prelude::*;
use log::error;
use rand::prelude::SliceRandom;
use serde::Serialize;
use uuid::Uuid;
use crate::model::database::{Group, GroupAssignment};
use crate::schema::group_assignments;
use crate::schema::group_assignments::{group_id, tickets};
use crate::schema::groups::dsl::groups;

pub fn generate_assignments(for_group_id: Uuid, connection: &mut PgConnection) -> Result<Vec<Uuid>, &str>
{
    let result = generate_assignments_for_group(for_group_id, connection);

    return match result
    {
        GenerateGroupAssignmentResult::Success(assignments) => Ok(assignments),
        GenerateGroupAssignmentResult::NotEnoughUsers => Err("Not enough users to generate assignments for group"),
        GenerateGroupAssignmentResult::UnknownDatabaseError(e) => {
            error!("{:?}", e);
            return Err("Unknown database error has occurred");
        }
    };
}

fn generate_assignments_for_group(group: Uuid, connection: &mut PgConnection) -> GenerateGroupAssignmentResult
{
    let assignments = groups
        .inner_join(group_assignments::table)
        .filter(group_id.eq(group))
        .filter(tickets.ne(0))
        .select((Group::as_select(), GroupAssignment::as_select()))
        .load::<(Group, GroupAssignment)>(connection)
        ;

    if let Err(e) = assignments
    {
        return GenerateGroupAssignmentResult::UnknownDatabaseError(e);
    }

    let assignments = assignments.unwrap();
    let positions = assignments[0].0.positions;
    let assignments: Vec<GroupAssignment> = assignments.into_iter().map(|a| a.1).collect();

    if assignments.len() < positions as usize
    {
        return GenerateGroupAssignmentResult::NotEnoughUsers;
    }

    let mut rng = rand::thread_rng();
    let mut available_tickets = Vec::new();

    for assignment in assignments
    {
        for _ in 0..assignment.tickets
        {
            available_tickets.push(assignment.user_id);
        }
    }

    let mut assignments = Vec::new();
    for _ in 0..positions
    {
        let user = available_tickets.choose(&mut rng).unwrap().clone();
        assignments.push(user);
        available_tickets.retain(|&u| u != user);
    }

    let result = diesel::update(group_assignments::table)
        .filter(group_id.eq(group))
        .filter(group_assignments::user_id.eq_any(assignments.clone()))
        .set(tickets.eq(0))
        .execute(connection);

    if let Err(e) = result
    {
        return GenerateGroupAssignmentResult::UnknownDatabaseError(e);
    }

    let result = diesel::update(group_assignments::table)
        .filter(group_id.eq(group))
        .filter(group_assignments::user_id.ne_all(assignments.clone()))
        .set(tickets.eq(tickets + 1))
        .execute(connection);

    if let Err(e) = result
    {
        return GenerateGroupAssignmentResult::UnknownDatabaseError(e);
    }

    return GenerateGroupAssignmentResult::Success(assignments.clone());
}

enum GenerateGroupAssignmentResult
{
    Success(Vec<Uuid>),
    NotEnoughUsers,
    UnknownDatabaseError(diesel::result::Error)
}

#[derive(Serialize, Debug)]
struct GroupAssignments
{
    group: Group,
    assignments: Vec<GroupAssignment>
}