use actix_web::{post, web::{self, Data}, HttpResponse, Responder};
use diesel::{r2d2::{ConnectionManager, Pool}, PgConnection};
use log::{error, warn};
use uuid::Uuid;

use crate::{actions::{self, data::get::group::GetGroupResult}, api::webhooks::{groups::events::Group, WebhookRequest}, model::database};

pub mod membership;
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

    let mut conn = pool.get_ref().get().expect("Error getting connection");
    let result = actions::data::save::group::to_database(&mut conn, &vec![group]);

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
    let mut pool = pool.get_ref().get().expect("Error getting connection");

    let payload = payload.into_inner();
    let updated = Group::from(payload);

    let update_result = pool
        .build_transaction()
        .read_write()
        .run(|conn| {
            let get_group_result = actions::data::get::group::by_planning_center_id(conn, &updated.data.id);
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
                return Err(diesel::result::Error::RollbackTransaction);
            }

            let group = group.unwrap();

            actions::data::save::group::to_database(conn, &vec![group])
        });

    match update_result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

#[post("/api/hooks/group/deleted")]
async fn group_deleted_webhook(
    payload: web::Json<WebhookRequest<String>>,
    pool: Data<Pool<ConnectionManager<PgConnection>>>
) -> impl Responder {
    let mut pool = pool.get_ref().get().expect("Error getting connection");

    let payload = payload.into_inner();
    let deleted = Group::from(payload);

    let delete_result = pool
        .build_transaction()
        .read_write()
        .run(|conn| {
            let get_group_result = actions::data::get::group::by_planning_center_id(conn, &deleted.data.id);
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
                    warn!("Group with planning center id: {} is already deleted", deleted.data.id);
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
                return Err(diesel::result::Error::RollbackTransaction);
            }

            let group = group.unwrap();
            actions::data::save::group::to_database(conn, &vec![group])
        });

    match delete_result {
        Ok(_) => HttpResponse::Ok(),
        Err(e) => {
            error!("{}", e);
            HttpResponse::InternalServerError()
        }
    }
}