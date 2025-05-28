use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddressRes {
    #[serde(rename = "level")]
    pub level: i32,

    #[serde(rename = "page")]
    pub page: i32,

    #[serde(rename = "limit")]
    pub limit: i32,

    #[serde(rename = "count")]
    pub count: i32,
    #[serde(rename = "addresses")]
    pub addresses: Vec<models::AddressResAddressesInner>,
}

impl AddressRes {
    pub fn new(
        level: i32,
        page: i32,
        limit: i32,
        count: i32,
        addresses: Vec<models::AddressResAddressesInner>,
    ) -> AddressRes {
        AddressRes {
            level,
            page,
            limit,
            count,
            addresses,
        }
    }
}
