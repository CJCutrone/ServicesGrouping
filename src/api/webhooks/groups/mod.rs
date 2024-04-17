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
        planning_center_id: created.data.id,
        is_deleted: false
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

    let get_group_result = actions::data::get::group::by_planning_center_id(pool.get_ref().clone(), &updated.data.id);

    let group = match get_group_result {
        GetGroupResult::UnknownDatabaseError(e) => {
            error!("{}", e);
            Err(())
        },
        GetGroupResult::MoreThanOneFound => {    
            error!("More than one group found with planning center id: {}", updated.data.id);
            Err(())
        },
        GetGroupResult::GroupDeleted => {
            warn!("Group with planning center id: {} is deleted", updated.data.id);
            Err(())
        },
        GetGroupResult::NotFound => {
            warn!("No group found with planning center id: {}", updated.data.id);
            
            let name = updated.data.attributes.unwrap().name;
            Ok(database::Group {
                id: Uuid::new_v5(&Uuid::NAMESPACE_OID, name.as_bytes()),
                name,
                positions: 0,
                planning_center_id: updated.data.id,
                is_deleted: false
            })
        },
        GetGroupResult::Success(mut group) => {
            group.name = updated.data.attributes.unwrap().name;
            Ok(group)
        }
    };

    if group.is_err() {
        return HttpResponse::InternalServerError();
    }

    match actions::data::save::group::to_database(pool.get_ref().clone(), &vec![group.unwrap()]) {
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
    pool: Data<Pool<ConnectionManager<PgConnection>>>
) -> impl Responder {
    let payload = payload.into_inner();
    let deleted = Group::from(payload);

    let get_group_result = actions::data::get::group::by_planning_center_id(pool.get_ref().clone(), &deleted.data.id);

    let group = match get_group_result {
        GetGroupResult::UnknownDatabaseError(e) => {
            error!("{}", e);
            Err(())
        },
        GetGroupResult::MoreThanOneFound => {    
            error!("More than one group found with planning center id: {}", deleted.data.id);
            Err(())
        },
        GetGroupResult::GroupDeleted => {
            error!("Group with planning center id: {} is already deleted", deleted.data.id);
            Err(())
        },
        GetGroupResult::NotFound => {
            error!("No group found with planning center id: {}", deleted.data.id);
            Err(())
        },
        GetGroupResult::Success(mut group) => {
            group.is_deleted = true;
            Ok(group)
        }
    };

    if group.is_err() {
        return HttpResponse::InternalServerError();
    }

    match actions::data::save::group::to_database(pool.get_ref().clone(), &vec![group.unwrap()]) {
        Ok(_) => HttpResponse::Ok(),
        Err(e) => {
            error!("{}", e);
            HttpResponse::InternalServerError()
        }
    }
}