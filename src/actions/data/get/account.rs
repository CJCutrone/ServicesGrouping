use diesel::prelude::*;
use diesel::{result::Error, PgConnection, SelectableHelper};
use uuid::Uuid;

use crate::model::database::Account;
use crate::schema::accounts::dsl::accounts;
use crate::schema::accounts::id;

pub fn by_id(
    conn: &mut PgConnection, 
    account_id: Uuid
) -> GetAccountResult {
    let result = accounts
        .filter(id.eq(account_id))
        .select(Account::as_select())
        .load(conn)
        ;

    if let Err(e) = result {
        return GetAccountResult::UnknownDatabaseError(e);
    }

    let account = result.unwrap();

    if account.is_empty() {
        return GetAccountResult::NotFound;
    }

    if account.len() > 1 {
        return GetAccountResult::MoreThanOneFound;
    }
    
    GetAccountResult::Success(account[0].clone())
}

pub enum GetAccountResult
{
    Success(Account),
    NotFound,
    MoreThanOneFound,
    UnknownDatabaseError(Error)
}