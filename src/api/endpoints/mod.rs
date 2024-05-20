use actix_web::{post, web::{self, Data}, HttpResponse, Responder};
use chrono::NaiveDateTime;
use diesel::{r2d2::{ConnectionManager, Pool}, PgConnection};
use uuid::Uuid;

use crate::{actions, api::requests::GenerateAssignmentRequests};


#[post("/api/groups/{id}/assign")]
async fn generate_assignments(
    path: web::Path<Uuid>,
    request: web::Json<GenerateAssignmentRequests>,
    db_pool: Data<Pool<ConnectionManager<PgConnection>>>
) -> impl Responder {
    let group_id = path.into_inner();
    let db_pool = db_pool.get_ref();

    let dates = request.dates.iter().map(|s| NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S").unwrap()).collect();

    let generate_assignments = actions::data::ticketing::generate_assignments_for_dates(
        group_id,
        dates,
        db_pool.clone()
    );

    HttpResponse::Ok().json(generate_assignments)
}

#[post("/api/schedule/configure")]
async fn configure_scheduler(
    db_pool: Data<Pool<ConnectionManager<PgConnection>>>
) -> impl Responder {

    //get database,
    //store the configuration in the database as json blob

    HttpResponse::Ok()
}

#[post("/api/schedule/{team}")]
async fn schedule (
    path: web::Path<String>,
    db_pool: Data<Pool<ConnectionManager<PgConnection>>>
) -> impl Responder {
    let team_id = path.into_inner();
    let db_pool = db_pool.get_ref();
        
    //get team positions for team with id
    //for each position, get people assigned
        //for each person, make sure they exist in our DB (if not, add them)
    //for each position assign people based on ticket data
    

    HttpResponse::Ok()
}