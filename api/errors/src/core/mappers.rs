use super::{Error, ErrorStatuses};
use sea_orm::DbErr;
use std::borrow::Cow;
use std::str::FromStr;

pub fn db_error_mapper(e: DbErr) -> Error {
    dbg!(&e);
    let basic_db_error = Error {
        error: ErrorStatuses::DatabaseError,
    };

    match e {
        DbErr::Custom(x) => Error {
            error: match ErrorStatuses::from_str(&x) {
                Ok(v) => v,
                Err(_) => ErrorStatuses::Unknown,
            },
        },
        DbErr::RecordNotFound(x) => Error {
            error: ErrorStatuses::NotFound { detail: x },
        },
        DbErr::Query(x) => match x {
            sea_orm::RuntimeErr::SqlxError(y) => match y.as_database_error() {
                Some(z) => match z.code() {
                    Some(c) => Error {
                        error: db_error_code_mapper(c, z.constraint().unwrap_or("")),
                    },
                    None => basic_db_error,
                },
                None => basic_db_error,
            },
            _ => basic_db_error,
        },
        DbErr::Exec(x) => match x {
            sea_orm::RuntimeErr::SqlxError(y) => match y.as_database_error() {
                Some(z) => match z.code() {
                    Some(c) => Error {
                        error: db_error_code_mapper(c, z.constraint().unwrap_or("")),
                    },
                    None => basic_db_error,
                },
                None => basic_db_error,
            },
            _ => basic_db_error,
        },
        _ => basic_db_error,
    }
}

fn db_error_code_mapper(code: Cow<'_, str>, constraint: &str) -> ErrorStatuses {
    match code.to_string().as_str() {
        "23503" => ErrorStatuses::DeletionForbidden {
            constraint: constraint.to_string(),
            detail: "".to_string(),
        },
        "23505" => ErrorStatuses::DuplicateKey {
            constraint: constraint.to_string(),
            detail: "".to_string(),
        },
        _ => ErrorStatuses::Generic {
            code: code.to_string(),
            constraint: constraint.to_string(),
            detail: "".to_string(),
        },
    }
}
