use actix_web::{post, web::{self, Data}, HttpResponse, Responder};
use diesel::{r2d2::{ConnectionManager, Pool}, PgConnection};
use log::{error, warn};
use uuid::Uuid;

use crate::{actions::{self, data::get::{group::GetGroupResult, user::GetUserResult}}, api::webhooks::{groups::membership::events::MembershipPayload, WebhookRequest}, model::database::{GroupAssignment, User}};

pub mod events;

#[post("/api/hooks/membership/created")]
async fn membership_created_webhook(
    payload: web::Json<WebhookRequest<String>>,
    pool: Data<Pool<ConnectionManager<PgConnection>>>
) -> impl Responder {
    let mut pool = pool.get_ref().get().expect("Error getting connection");

    let payload = payload.into_inner();
    let membership = MembershipPayload::from(payload);
    let person = membership.data.relationships.person.data;
    let group = membership.data.relationships.group.data;

    let created_result = pool
        .build_transaction()
        .read_write()
        .run(|conn| {
            let get_group_result = actions::data::get::group::by_planning_center_id(conn, &group.id);
            let group = match get_group_result {
                GetGroupResult::UnknownDatabaseError(e) => {
                    error!("{}", e);
                    Err(())
                },
                GetGroupResult::MoreThanOneFound => {    
                    error!("More than one group found with planning center id: {}", group.id);
                    Err(())
                },
                GetGroupResult::GroupDeleted => {
                    error!("Group with planning center id: {} is deleted", group.id);
                    Err(())
                },
                GetGroupResult::NotFound => {
                    error!("No group found with planning center id: {}", group.id);
                    Err(())
                },
                GetGroupResult::Success(group) => Ok(group)
            };

            if group.is_err() {
                return Err(diesel::result::Error::RollbackTransaction);
            }

            let group = group.unwrap();

            let get_user_result = actions::data::get::user::by_planning_center_id(conn, &person.id);
            let person = match get_user_result {
                GetUserResult::UnknownDatabaseError(e) => {
                    error!("{}", e);
                    Err(())
                },
                GetUserResult::MoreThanOneFound => {
                    error!("More than one person found with planning center id: {}", person.id);
                    Err(())
                },
                GetUserResult::UserDeleted => {
                    error!("Person with planning center id: {} is deleted", person.id);
                    Err(())
                },
                GetUserResult::NotFound => {
                    warn!("No person found with planning center id: {}", person.id);

                    //TODO: once client is built out, go get person info from planning center
                    let user = User {
                        id: Uuid::new_v5(&Uuid::NAMESPACE_OID, "--fake--".as_bytes()),
                        first_name: "".to_string(),
                        last_name: "".to_string(),
                        planning_center_id: person.id,
                        is_deleted: false                    
                    };

                    actions::data::save::user::to_database(conn, &vec![user.clone()]);

                    Ok(user)
                },
                GetUserResult::Success(person) => Ok(person)
            };

            if person.is_err() {
                return Err(diesel::result::Error::RollbackTransaction);
            }

            let person = person.unwrap();

            let group_assignment = GroupAssignment {
                id: Uuid::new_v5(&Uuid::NAMESPACE_OID, format!("{}{}", person.id, group.id).as_bytes()),
                group_id: group.id,
                user_id: person.id,
                tickets: 1
            };

            actions::data::save::group_assignment::to_database(conn, &vec![group_assignment]);

            Ok(())
        });

    match created_result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::Ok().finish()
    }
}