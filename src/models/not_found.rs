use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotFound {
    #[serde(rename = "request_id")]
    pub request_id: String,

    #[serde(rename = "error_code")]
    pub error_code: String,

    #[serde(rename = "message")]
    pub message: String,
}

impl NotFound {
    pub fn new(request_id: String, error_code: String, message: String) -> NotFound {
        NotFound {
            request_id,
            error_code,
            message,
        }
    }
}
