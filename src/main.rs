use std::env;
use std::io::{Error, ErrorKind};
use actix_cors::Cors;
use actix_session::config::PersistentSession;
use actix_session::SessionMiddleware;
use actix_session::storage::CookieSessionStore;
use actix_web::{App, cookie, HttpServer};
use actix_web::cookie::Key;
use actix_web::web::Data;
use log::{error, info};
use dotenv::dotenv;

use crate::actions::data::get_db_connection;

pub mod actions;
pub mod api;
pub mod model;
pub mod oauth;
pub mod planning_center;
pub mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    pretty_env_logger::init();

    let pool = get_db_connection();
    if let Err(e) = pool {
        error!("{:?}", e);
        return Err(Error::new(ErrorKind::Other, "Failed to establish connection to database"));
    }
    let pool = pool.unwrap();
    info!("Connection pool established");

    info!("API command received");
    let domain = env::var("SERVER_DOMAIN").expect("SERVER_DOMAIN must be set");
    let planning_center_id = env::var("PLANNING_CENTER_ID").expect("PLANNING_CENTER_ID must be set");
    let planning_center_secret = env::var("PLANNING_CENTER_SECRET").expect("PLANNING_CENTER_SECRET must be set");

    let configuration = ApplicationConfiguration {
        domain,
        planning_center_id,
        planning_center_secret
    };

    return HttpServer::new(move ||
        App::new()
            .wrap(
                //TODO: once initial development is done, widdle down what is allowed
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .supports_credentials()
                    .max_age(3600)
            )
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
                    .cookie_secure(true)
                    .session_lifecycle(
                        PersistentSession::default().session_ttl(cookie::time::Duration::hours(2)),
                    )
                    .build()
            )
            .service(api::endpoints::generate_assignments)
            .service(api::webhooks::groups::group_created_webhook)
            .service(api::webhooks::groups::group_updated_webhook)
            .service(api::webhooks::groups::group_deleted_webhook)
            .service(api::webhooks::groups::membership::membership_created_webhook)
            .service(oauth::endpoints::callback)
            .service(oauth::endpoints::me)
            .service(oauth::endpoints::refresh_token)
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(configuration.clone()))
        )
        .bind("0.0.0.0:8080")?
        .run()
        .await;
}

#[derive(Clone)]
pub struct ApplicationConfiguration {
    pub domain: String,
    pub planning_center_id: String,
    pub planning_center_secret: String
}