use mysql::{self, FromRowError, Row};
use mysql::prelude::FromRow;

use rocket::{State, Request};
use rocket::request::{Outcome, FromRequest};
use rocket::outcome::{IntoOutcome};
use database;

#[derive(Serialize)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub passhash: String,
    pub salt: String,
    pub apikey: String
}

#[derive(Serialize)]
pub struct Record {
    pub name: String,
    pub token: String,
    pub expiration: String,
    pub apikey: String
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<User, ()> {
        let pool = request.guard::<State<mysql::Pool>>()?;

        request.cookies()
            .get_private("usertoken")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(|token| database::get_user(pool.inner(), token).unwrap())
            .or_forward(())
    }
}

impl FromRow for User {
    fn from_row(row: Row) -> User {
        Self::from_row_opt(row).unwrap()
    }

    fn from_row_opt(row: Row) -> Result<User, FromRowError> {
        if row.len() != 5 {
            return Err(FromRowError(row))
        }

        Ok(User {
            id: row.get(0).unwrap(),
            username: row.get(1).unwrap(),
            passhash: row.get(2).unwrap(),
            salt: row.get(3).unwrap(),
            apikey: row.get(4).unwrap()
        })
    }
}

impl FromRow for Record {
    fn from_row(row: Row) -> Record {
        Self::from_row_opt(row).unwrap()
    }

    fn from_row_opt(row: Row) -> Result<Record, FromRowError> {
        if row.len() < 4 {
            return Err(FromRowError(row))
        }

        Ok(Record {
            name: row.get(0).unwrap(),
            token: row.get(1).unwrap(),
            expiration: row.get(2).unwrap(),
            apikey: row.get(3).unwrap()
        })
    }
}
