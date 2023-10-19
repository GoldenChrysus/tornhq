use actix_web::{error, http::StatusCode, HttpResponse};
use derive_more::{Display, Error as DeriveError};
use errors::core::{Error, ErrorStatuses};
use serde::Serialize;

#[derive(Debug, Display, Serialize, DeriveError)]
pub struct APIError {
    pub error: ErrorStatuses,
}

impl error::ResponseError for APIError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .content_type("application/json")
            .json(&self)
    }

    fn status_code(&self) -> StatusCode {
        match &self.error {
            ErrorStatuses::CreationForbidden { .. } => StatusCode::BAD_REQUEST,
            ErrorStatuses::DatabaseError => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorStatuses::DeletionForbidden { .. } => StatusCode::METHOD_NOT_ALLOWED,
            ErrorStatuses::DuplicateKey { .. } => StatusCode::BAD_REQUEST,
            ErrorStatuses::DuplicateSequence => StatusCode::BAD_REQUEST,
            ErrorStatuses::Generic { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorStatuses::InvalidContent => StatusCode::BAD_REQUEST,
            ErrorStatuses::InvalidInput => StatusCode::BAD_REQUEST,
            ErrorStatuses::InvalidPath => StatusCode::NOT_FOUND,
            ErrorStatuses::NotFound { .. } => StatusCode::NOT_FOUND,
            ErrorStatuses::RemoteError => StatusCode::FAILED_DEPENDENCY,
            ErrorStatuses::SystemError => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorStatuses::Unauthorized => StatusCode::UNAUTHORIZED,
            ErrorStatuses::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorStatuses::UpdateForbidden { .. } => StatusCode::METHOD_NOT_ALLOWED,
            ErrorStatuses::ValueError { .. } => StatusCode::BAD_REQUEST,
        }
    }
}

pub fn make_error(status: ErrorStatuses) -> APIError {
    APIError { error: status }
}

pub fn error_mapper(e: Error) -> APIError {
    dbg!(&e);
    APIError { error: e.error }
}
