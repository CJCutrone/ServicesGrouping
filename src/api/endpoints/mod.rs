use actix_web::{get, post, web::{self, Data}, HttpResponse, Responder};
use chrono::NaiveDateTime;
use diesel::{r2d2::{ConnectionManager, Pool}, PgConnection};
use log::{error, info};
use uuid::Uuid;

use crate::{actions::{self, data::get::planning_center_access_tokens::{get_decrypted_tokens_for_account, DecryptAccountTokensResult}}, api::requests::GenerateAssignmentRequests, model::database::PlanningCenterAccessTokens, planning_center, ApplicationConfiguration};


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
    let _db_pool = db_pool.get_ref();
    //get database,
    //store the configuration in the database as json blob

    HttpResponse::Ok()
}

#[get("/api/schedule/{team}")]
async fn schedule (
    account: PlanningCenterAccessTokens,
    path: web::Path<String>,
    db_pool: Data<Pool<ConnectionManager<PgConnection>>>,
    configuration: Data<ApplicationConfiguration>
) -> impl Responder {
    let team_id = path.into_inner();
    let mut conn = db_pool.get_ref().get().expect("Error getting connection");
    let configuration = configuration.get_ref();
        
    let account = match get_decrypted_tokens_for_account(account.id, configuration, &mut conn) {
        DecryptAccountTokensResult::Success(account) => account,
        _ => {
            error!("Failed to get account tokens");
            return HttpResponse::InternalServerError().finish();
        }
    };

    let team_positions = planning_center::services::v2::teams::team_positions(&team_id, &account.access_token).await;

    if team_positions.is_err() {
        error!("Failed to get team positions");
        return HttpResponse::InternalServerError().finish();
    }

    info!("Acquired team positions");
    let team_positions = team_positions.unwrap().data;

    //foreach each team_position
    for position in team_positions {
        let team_assignments = planning_center::services::v2::teams::team_positions::person_team_position_assignments(
            &team_id, 
            &position.id, 
            &account.access_token
        ).await;

        info!("Acquired team assignments");

        if team_assignments.is_err() {
            error!("Failed to get team assignments");
            return HttpResponse::InternalServerError().finish();
        }

        let team_assignments = team_assignments.unwrap().data;
        for assignment in team_assignments {
            info!("Person Assigned: {:?}", assignment.relationships.person.data.id);
        }
    }

    //get team positions for team with id
    //for each position, get people assigned
        //for each person, make sure they exist in our DB (if not, add them)
    //for each position assign people based on ticket data
    

    HttpResponse::Ok().finish()
}