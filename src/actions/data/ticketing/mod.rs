use std::collections::HashMap;
use chrono::NaiveDateTime;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use log::{error, trace};
use rand::Rng;
use serde::Serialize;
use uuid::Uuid;
use crate::model::database::{Group, GroupAssignment, ServiceDate};
use crate::schema::{group_assignments, service_dates};
use crate::schema::group_assignments::{group_id, tickets};
use crate::schema::groups::dsl::groups;

pub fn generate_assignments_for_dates(
    for_group_id: Uuid,
    dates: Vec<NaiveDateTime>,
    pool: Pool<ConnectionManager<PgConnection>>
) -> Result<Vec<Assignment>, &'static str>
{
    let assignments = generate_assignments(for_group_id, pool.clone())?;

    trace!("Building service_date_assignments for selected users");

    let mut date_assignments = Vec::new();
    for date in dates.iter()
    {
        let assignments_for_date: Vec<ServiceDate> = assignments.iter().map(|a| ServiceDate {
            id: Uuid::new_v5(&Uuid::NAMESPACE_OID, format!("{}{}", a.group_assignment_id, *date).as_bytes()),
            group_assignment_id: a.group_assignment_id,
            for_date: *date,
            tickets_consumed: a.tickets_consumed
        }).collect();

        date_assignments.extend(assignments_for_date);
    }

    let mut connection = pool.get().expect("Error getting connection");

    for date_assignment in date_assignments {
        insert_into(service_dates::table)
            .values(date_assignment)
            .on_conflict(service_dates::id)
            .do_nothing()
            .execute(&mut connection)
            .expect("Failed to insert group assignments");
    }

    trace!("Completed assignment");
    Ok(assignments)
}

pub fn generate_assignments(for_group_id: Uuid, pool: Pool<ConnectionManager<PgConnection>>) -> Result<Vec<Assignment>, &'static str>
{
    let mut connection = pool.get().expect("Error getting connection");
    let result = generate_assignments_for_group(for_group_id, &mut connection);

    match result
    {
        GenerateGroupAssignmentResult::Success(assignments) => Ok(assignments),
        GenerateGroupAssignmentResult::NotEnoughUsers => {
            error!("Not enough users to generate assignments for group");
            Err("Not enough users to generate assignments for group")
        },
        GenerateGroupAssignmentResult::UnknownDatabaseError(e) => {
            error!("{:?}", e);
            Err("Unknown database error has occurred")
        }
    }
}

fn generate_assignments_for_group(group: Uuid, connection: &mut PgConnection) -> GenerateGroupAssignmentResult
{
    trace!("Generating assignments");
    trace!("Gathering assignment pool");
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

    trace!("Building ticket pool");
    let assignments = assignments.unwrap();
    let positions = assignments[0].0.positions;
    let assignments: Vec<GroupAssignment> = assignments.into_iter().map(|a| a.1).collect();

    if assignments.len() < positions as usize
    {
        return GenerateGroupAssignmentResult::NotEnoughUsers;
    }

    let mut rng = rand::thread_rng();
    let mut available_tickets = Vec::new();
    let tickets_per_user = assignments.iter().map(|a| (a.user_id, a.tickets)).collect::<HashMap<Uuid, i32>>();

    for assignment in assignments
    {
        for _ in 0..assignment.tickets
        {
            available_tickets.push(Assignment {
                group_assignment_id: assignment.id,
                group_id: assignment.group_id,
                user_id: assignment.user_id,
                tickets_consumed: *tickets_per_user.get(&assignment.user_id).unwrap()
            });
        }
    }

    trace!("Selecting tickets");
    let mut assignments: Vec<Assignment> = Vec::new();
    for _ in 0..positions
    {
        let index = rng.gen_range(0..available_tickets.len());
        let assignment = available_tickets[index];        
        assignments.push(assignment);

        available_tickets.retain(
            |u| u.user_id != assignment.user_id
        );
    }

    let assigned_users = assignments.iter().map(|a| a.user_id).collect::<Vec<Uuid>>();

    trace!("Zeroing out tickets for assigned users");
    let result = diesel::update(group_assignments::table)
        .filter(group_id.eq(group))
        .filter(group_assignments::user_id.eq_any(assigned_users.clone()))
        .set(tickets.eq(0))
        .execute(connection);

    if let Err(e) = result
    {
        return GenerateGroupAssignmentResult::UnknownDatabaseError(e);
    }

    trace!("Adding tickets to non-assigned users");
    let result = diesel::update(group_assignments::table)
        .filter(group_id.eq(group))
        .filter(group_assignments::user_id.ne_all(assigned_users.clone()))
        .set(tickets.eq(tickets + 1))
        .execute(connection);

    if let Err(e) = result
    {
        return GenerateGroupAssignmentResult::UnknownDatabaseError(e);
    }

    GenerateGroupAssignmentResult::Success(assignments)
}

enum GenerateGroupAssignmentResult
{
    Success(Vec<Assignment>),
    NotEnoughUsers,
    UnknownDatabaseError(diesel::result::Error)
}

#[derive(Serialize, Debug)]
struct GroupAssignments
{
    group: Group,
    assignments: Vec<GroupAssignment>
}

#[derive(Debug, Serialize, Clone, Copy)]
pub struct Assignment {
    pub group_assignment_id: Uuid,
    pub user_id: Uuid,
    pub group_id: Uuid,
    pub tickets_consumed: i32
}