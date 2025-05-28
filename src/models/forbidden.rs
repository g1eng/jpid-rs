use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Forbidden {
    #[serde(rename = "request_id")]
    pub request_id: String,

    #[serde(rename = "error_code")]
    pub error_code: String,

    #[serde(rename = "message")]
    pub message: String,
}

impl Forbidden {
    pub fn new(request_id: String, error_code: String, message: String) -> Forbidden {
        Forbidden {
            request_id,
            error_code,
            message,
        }
    }
}
