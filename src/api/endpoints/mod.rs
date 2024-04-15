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