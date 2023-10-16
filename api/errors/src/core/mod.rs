use derive_more::Display;
use display_json::{DisplayAsJson, FromStrAsJson};
use serde::{Deserialize, Serialize};

pub mod mappers;

#[derive(Serialize, Clone, Debug, Display)]
pub struct Error {
    pub error: ErrorStatuses,
}

#[derive(Serialize, Deserialize, Clone, Debug, DisplayAsJson, FromStrAsJson)]
pub enum ErrorStatuses {
    CreationForbidden {
        constraint: String,
        detail: String,
    },
    DatabaseError,
    DeletionForbidden {
        constraint: String,
        detail: String,
    },
    DuplicateKey {
        constraint: String,
        detail: String,
    },
    DuplicateSequence,
    Generic {
        code: String,
        constraint: String,
        detail: String,
    },
    InvalidContent,
    InvalidInput,
    InvalidPath,
    NotFound {
        detail: String,
    },
    RemoteError,
    SystemError,
    Unauthorized,
    Unknown,
    UpdateForbidden {
        constraint: String,
        detail: String,
    },
    ValueError {
        field: String,
        reason: String,
    },
}

pub fn make_error(status: ErrorStatuses) -> Error {
    Error { error: status }
}

#[derive(Serialize, Debug, Clone, Display)]
pub enum ErrorCodes {
    AlreadyApproved,
    FieldInvalid,
    FieldMissing,
    NegativeValue,
    NotApproved,
    PositiveValue,
    RecordNotFound,

    NONE,
}
