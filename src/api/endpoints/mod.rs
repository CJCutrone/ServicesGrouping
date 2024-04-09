use actix_web::{post, web::{self, Data}, HttpResponse, Responder};
use diesel::{r2d2::{ConnectionManager, Pool}, PgConnection};
use uuid::Uuid;

use crate::actions;


#[post("/groups/{id}/assign")]
async fn generate_assignments(
    path: web::Path<Uuid>,
    db_pool: Data<Pool<ConnectionManager<PgConnection>>>
) -> impl Responder {
    let group_id = path.into_inner();
    let db_pool = db_pool.get_ref();

    let generate_assignments = actions::data::ticketing::generate_assignments(
        group_id,
        db_pool.clone()
    );

    HttpResponse::Ok().json(generate_assignments)
}