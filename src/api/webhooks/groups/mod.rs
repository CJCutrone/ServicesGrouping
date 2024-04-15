use actix_web::{post, web::{self, Data}, HttpResponse, Responder};
use diesel::{r2d2::{ConnectionManager, Pool}, PgConnection};
use uuid::Uuid;

use crate::{actions, api::webhooks::{groups::events::Group, WebhookRequest}, model::database};

pub mod events;

#[post("/api/hooks/group/created")]
async fn group_created_webhook(
    payload: web::Json<WebhookRequest<String>>,
    pool: Data<Pool<ConnectionManager<PgConnection>>>
) -> impl Responder {
    let payload = payload.into_inner();
    let created = Group::from(payload);

    let name = created.data.attributes.unwrap().name;
    let group = database::Group {
        id: Uuid::new_v5(&Uuid::NAMESPACE_OID, name.as_bytes()),
        name,
        positions: 0,
        planning_center_id: created.data.id
    };

    actions::data::save::group::to_database(pool.get_ref().clone(), &vec![group]);

    HttpResponse::Ok()
}

#[post("/api/hooks/group/updated")]
async fn group_updated_webhook(
    payload: web::Json<WebhookRequest<String>>,
    _: Data<Pool<ConnectionManager<PgConnection>>>
) -> impl Responder {
    let payload = payload.into_inner();
    let group = Group::from(payload);
    println!("{:?}", group);

    HttpResponse::Ok()
}

#[post("/api/hooks/group/deleted")]
async fn group_deleted_webhook(
    payload: web::Json<WebhookRequest<String>>,
    _: Data<Pool<ConnectionManager<PgConnection>>>
) -> impl Responder {
    let payload = payload.into_inner();
    let group = Group::from(payload);
    println!("{:?}", group);

    HttpResponse::Ok()
}