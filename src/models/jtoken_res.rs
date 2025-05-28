use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JtokenRes {
    #[serde(rename = "token")]
    pub token: String,

    #[serde(rename = "token_type")]
    pub token_type: String,

    #[serde(rename = "expires_in")]
    pub expires_in: i32,

    #[serde(rename = "scope")]
    pub scope: String,
}

impl JtokenRes {
    pub fn new(token: String, token_type: String, expires_in: i32, scope: String) -> JtokenRes {
        JtokenRes {
            token,
            token_type,
            expires_in,
            scope,
        }
    }
}
