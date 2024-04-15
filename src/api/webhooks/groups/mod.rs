use actix_web::{post, web::{self, Data}, HttpResponse, Responder};
use diesel::{r2d2::{ConnectionManager, Pool}, PgConnection};

use crate::api::webhooks::{groups::events::Group, WebhookRequest};

pub mod events;

#[post("/api/hooks/group/created")]
async fn group_created_webhook(
    payload: web::Json<WebhookRequest<String>>,
    _: Data<Pool<ConnectionManager<PgConnection>>>
) -> impl Responder {
    let payload = payload.into_inner();
    let group = Group::from(payload);
    println!("{:?}", group);

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