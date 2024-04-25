use std::{future::Future, pin::Pin};

use actix_session::SessionExt;
use actix_web::{dev::Payload, error::ErrorUnauthorized, Error, HttpRequest};

use crate::model::database::Account;

impl actix_web::FromRequest for Account
{
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Account, Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let session = req.get_session();
        let account = session.get::<Account>("account");

        Box::pin(async move {
            if let Ok(Some(account)) = account {
                return Ok(account);
            }

            Err(ErrorUnauthorized("Unauthorized"))
        })
    }
}