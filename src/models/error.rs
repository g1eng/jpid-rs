use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Error {
    #[serde(rename = "request_id")]
    pub request_id: String,

    #[serde(rename = "error_code")]
    pub error_code: String,

    #[serde(rename = "message")]
    pub message: String,
}

impl Error {
    pub fn new(request_id: String, error_code: String, message: String) -> Error {
        Error {
            request_id,
            error_code,
            message,
        }
    }
}
