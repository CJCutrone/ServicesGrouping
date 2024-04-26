use std::{future::Future, pin::Pin};

use actix_session::SessionExt;
use actix_web::{dev::Payload, error::ErrorUnauthorized, Error, HttpRequest};

use crate::model::database::PlanningCenterAccessTokens;

impl actix_web::FromRequest for PlanningCenterAccessTokens
{
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<PlanningCenterAccessTokens, Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let session = req.get_session();
        let account = session.get::<PlanningCenterAccessTokens>("access_tokens");

        Box::pin(async move {
            if let Ok(Some(account)) = account {
                return Ok(account);
            }

            Err(ErrorUnauthorized("Unauthorized"))
        })
    }
}