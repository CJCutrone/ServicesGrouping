use actix_web::{post, web::{self, Data}, HttpResponse, Responder};
use diesel::{r2d2::{ConnectionManager, Pool}, PgConnection};
use log::{error, warn};
use uuid::Uuid;

use crate::{actions::{self, data::get::group::GetGroupResult}, api::webhooks::{groups::events::Group, WebhookRequest}, model::database};

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

    let result = actions::data::save::group::to_database(pool.get_ref().clone(), &vec![group]);

    match result {
        Ok(_) => HttpResponse::Ok(),
        Err(e) => {
            error!("{}", e);
            HttpResponse::InternalServerError()
        }
    }
}

#[post("/api/hooks/group/updated")]
async fn group_updated_webhook(
    payload: web::Json<WebhookRequest<String>>,
    pool: Data<Pool<ConnectionManager<PgConnection>>>
) -> impl Responder {
    let payload = payload.into_inner();
    let updated = Group::from(payload);
    let pool = pool.get_ref();

    let result = actions::data::get::group::by_planning_center_id(
        pool.clone(), 
        &updated.data.id
    );

    let group = match result {
        GetGroupResult::UnknownDatabaseError(e) => {
            error!("{}", e);
            return HttpResponse::InternalServerError();
        },
        GetGroupResult::MoreThanOneFound => {    
            error!("More than one group found with planning center id: {}", updated.data.id);
            return HttpResponse::InternalServerError();
        },
        GetGroupResult::NotFound => {
            warn!("No group found with planning center id: {}", updated.data.id);
            
            let name = updated.data.attributes.unwrap().name;
            database::Group {
                id: Uuid::new_v5(&Uuid::NAMESPACE_OID, name.as_bytes()),
                name,
                positions: 0,
                planning_center_id: updated.data.id
            }
        },
        GetGroupResult::Success(mut group) => {
            group.name = updated.data.attributes.unwrap().name;
            group
        }
    };

    let result = actions::data::save::group::to_database(pool.clone(), &vec![group]);

    match result {
        Ok(_) => HttpResponse::Ok(),
        Err(e) => {
            error!("{}", e);
            HttpResponse::InternalServerError()
        }
    }
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