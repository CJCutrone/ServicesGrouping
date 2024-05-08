use actix_web::{post, web::{self, Data}, HttpResponse, Responder};
use diesel::{r2d2::{ConnectionManager, Pool}, PgConnection};
use log::{error, info, trace, warn};
use uuid::Uuid;

use crate::{actions::{self, data::{get::{group::GetGroupResult, planning_center_access_tokens::{get_decrypted_tokens_for_account, DecryptAccountTokensResult}, user::GetUserResult}, save::planning_center_access_tokens::encrypt_and_store_tokens}}, api::webhooks::{groups::membership::events::MembershipPayload, WebhookRequest}, model::database::{self, Group, GroupAssignment, PlanningCenterAccessTokens, User}, planning_center::{self, oauth::refresh_token}, ApplicationConfiguration};

use self::events::RelationshipData;

pub mod events;

#[post("/api/hooks/{owner_id}/membership/created")]
async fn membership_created_webhook(
    configuration: Data<ApplicationConfiguration>,
    payload: web::Json<WebhookRequest<String>>,
    path: web::Path<Uuid>,
    pool: Data<Pool<ConnectionManager<PgConnection>>>
) -> impl Responder {
    let mut conn = pool.get_ref().get().expect("Error getting connection");
    let configuration = configuration.get_ref();

    let account_id = path.into_inner();
    let payload = payload.into_inner();
    let membership = MembershipPayload::from(payload);

    let person = membership.data.relationships.person.data;
    let group = membership.data.relationships.group.data;

    trace!("Getting access tokens from DB");
    //get decrypted tokens from database
    let account_tokens = match get_decrypted_tokens_for_account(account_id, configuration, &mut conn) {
        DecryptAccountTokensResult::Success(tokens) => Ok(tokens),
        _ => Err(())
    };

    if account_tokens.is_err() {
        return HttpResponse::InternalServerError().finish();        
    }

    let account_tokens = account_tokens.unwrap();

    trace!("Getting group from DB or Planning Center API");
    //get group from database or planning center API.
    let group = get_group_from_database_or_planning_center(account_tokens.clone(), configuration, &group, &mut conn).await;

    if group.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    let group = group.unwrap();

    trace!("Getting person from DB or Planning Center API");
    let person = get_person_from_database_or_planning_center(account_tokens, configuration, &person, &mut conn).await;

    if person.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    let person = person.unwrap();

    trace!("Save group assignment to DB");
    //save group assignment
    let data = GroupAssignment {
        id: Uuid::new_v5(&Uuid::NAMESPACE_OID, format!("{}{}", person.id, group.id).as_bytes()),
        group_id: group.id,
        user_id: person.id,
        tickets: 1
    };
    actions::data::save::group_assignment::to_database(&mut conn, &vec![data]);

    HttpResponse::Ok().finish()
}

async fn get_person_from_database_or_planning_center(
    tokens: PlanningCenterAccessTokens,
    configuration: &ApplicationConfiguration,
    person: &RelationshipData,
    conn: &mut PgConnection
) -> Result<User,()>{
    let account_tokens = refresh_token_if_expired(tokens, configuration, conn).await;

    if account_tokens.is_err() {
        return Err(());
    }

    let account_tokens = account_tokens.unwrap();

    let get_user_result = actions::data::get::user::by_planning_center_id(conn, &person.id);

    match get_user_result {
        GetUserResult::UnknownDatabaseError(e) => {
            error!("{}", e);
            Err(())
        },
        GetUserResult::MoreThanOneFound => {    
            error!("More than one user found with planning center id: {}", person.id);
            Err(())
        },
        GetUserResult::UserDeleted => {
            error!("User with planning center id: {} is deleted", person.id);
            Err(())
        },
        GetUserResult::NotFound => {
            info!("No user found with planning center id: {}", person.id);
            match planning_center::people::v2::people(person.id.clone(), account_tokens.access_token).await {
                Ok(person) => {
                    let data = person.data;
                    let first_name = data.attributes.first_name;
                    let last_name = data.attributes.last_name;


                    let user = User {
                        id: Uuid::new_v5(&Uuid::NAMESPACE_OID, data.id.as_bytes()),
                        planning_center_id: data.id,
                        first_name,
                        last_name,
                        is_deleted: false
                    };

                    actions::data::save::user::to_database(conn, &vec![database::User {
                        id: user.id,
                        planning_center_id: user.planning_center_id.clone(),
                        first_name: user.first_name.clone(),
                        last_name: user.last_name.clone(),
                        is_deleted: user.is_deleted
                    }]);

                    Ok(user)
                },
                Err(e) => {
                    error!("{:?}", e);
                    Err(())
                }
            }
        },
        GetUserResult::Success(person) => Ok(person)
    }
}

async fn get_group_from_database_or_planning_center(
    tokens: PlanningCenterAccessTokens,
    configuration: &ApplicationConfiguration,
    group: &RelationshipData,
    conn: &mut PgConnection
) -> Result<Group,()>{
    let account_tokens = refresh_token_if_expired(tokens, configuration, conn).await;

    if account_tokens.is_err() {
        return Err(());
    }

    let account_tokens = account_tokens.unwrap();

    let get_group_result = actions::data::get::group::by_planning_center_id(conn, &group.id);

    match get_group_result {
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
            warn!("No group found with planning center id: {}", group.id);
            match planning_center::group::v2::groups(group.id.clone(), account_tokens.access_token).await {
                Ok(group) => {
                    let data = group.data;
                    let group_name = data.attributes.name;

                    let group = Group {
                        id: Uuid::new_v5(&Uuid::NAMESPACE_OID, group_name.as_bytes()),
                        planning_center_id: data.id,
                        name: group_name,
                        positions: 0,
                        is_deleted: false
                    };

                    let _ = actions::data::save::group::to_database(conn, &vec![database::Group {
                        id: group.id,
                        planning_center_id: group.planning_center_id.clone(),
                        name: group.name.clone(),
                        positions: group.positions,
                        is_deleted: group.is_deleted
                    }]).expect("Was not able to save group to database");

                    Ok(group)
                },
                Err(e) => {
                    error!("{:?}", e);
                    Err(())
                }
            }
        },
        GetGroupResult::Success(group) => Ok(group)
    }
}


async fn refresh_token_if_expired(
    account_tokens: PlanningCenterAccessTokens,
    configuration: &ApplicationConfiguration,
    conn: &mut PgConnection
) -> Result<PlanningCenterAccessTokens, ()> {
    let now = chrono::Utc::now().timestamp();
    if account_tokens.expires_at < now {
        let refreshed_token = refresh_token(configuration, account_tokens.refresh_token).await;
        
        if let Err(e) = refreshed_token {
            error!("{:?}", e.error);
            return Err(());
        }

        let refreshed_token = refreshed_token.unwrap();

        let account_tokens = PlanningCenterAccessTokens {
            id: account_tokens.id,
            planning_center_id: account_tokens.planning_center_id.clone(),
            access_token: refreshed_token.access_token.clone(),
            refresh_token: refreshed_token.refresh_token.clone(),
            expires_at: refreshed_token.expires_at
        };

        let _ = encrypt_and_store_tokens(&account_tokens, configuration, conn);

        Ok(account_tokens)
    } else {
        Ok(account_tokens)
    }
}

/*
    let get_user_result = actions::data::get::user::by_planning_center_id(&mut conn, &person.id);
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

            let configuration = configuration.get_ref();

            let planning_center_id = "--fake--".to_string();
            let account_id = Uuid::new_v5(&Uuid::NAMESPACE_OID, planning_center_id.as_bytes());

            let user: Result<User, ()> = match get_decrypted_tokens_for_account(account_id, configuration, &mut conn) {
                DecryptAccountTokensResult::Success(tokens) => {
                    let refreshed_token = match refresh_token(configuration, tokens.refresh_token).await {
                        Err(e) => {
                            error!("{:?}", e.error);
                            Err(())
                        },
                        Ok(response) => {
                            let planning_center_access_tokens = PlanningCenterAccessTokens {
                                id: account_id,
                                planning_center_id,
                                access_token: response.access_token.clone(),
                                refresh_token: response.refresh_token.clone(),
                                expires_at: response.expires_at
                            };
                
                            let tokens = encrypt_and_store_tokens(planning_center_access_tokens, configuration, &mut conn);
                            
                            Ok(planning_center_access_tokens)
                        }
                    };

                    if refreshed_token.is_err() {
                        return HttpResponse::InternalServerError().finish();
                    }

                    let refreshed_token = refreshed_token.unwrap();

                    people::v2::people(planning_center_id, refreshed_token.access_token);

                    // HttpResponse::Ok().json(AccountTokens {
                    //     access_token: tokens.access_token,
                    //     refresh_token: tokens.refresh_token,
                    //     expires_at: tokens.expires_at
                    // })
                    Ok(User {
                        id: Uuid::new_v5(&Uuid::NAMESPACE_OID, "--fake--".as_bytes()),
                        first_name: "".to_string(),
                        last_name: "".to_string(),
                        planning_center_id: person.id,
                        is_deleted: false                    
                    })
                },
                _ => Err(())
            };

            if user.is_err() {
                return HttpResponse::InternalServerError().finish();
            }

            let user = user.unwrap();

            actions::data::save::user::to_database(&mut conn, &vec![user.clone()]);

            Ok(user)
        },
        GetUserResult::Success(person) => Ok(person)
    };

    if person.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    let person = person.unwrap();

    let group_assignment = GroupAssignment {
        id: Uuid::new_v5(&Uuid::NAMESPACE_OID, format!("{}{}", person.id, group.id).as_bytes()),
        group_id: group.id,
        user_id: person.id,
        tickets: 1
    };

    actions::data::save::group_assignment::to_database(&mut conn, &vec![group_assignment]);

    HttpResponse::Ok().finish()
}
*/