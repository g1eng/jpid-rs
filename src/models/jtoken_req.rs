use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JtokenReq {
    #[serde(rename = "grant_type")]
    pub grant_type: String,

    #[serde(rename = "client_id")]
    pub client_id: String,

    #[serde(rename = "secret_key")]
    pub secret_key: String,
}

impl JtokenReq {
    pub fn new(grant_type: String, client_id: String, secret_key: String) -> JtokenReq {
        JtokenReq {
            grant_type,
            client_id,
            secret_key,
        }
    }
}
