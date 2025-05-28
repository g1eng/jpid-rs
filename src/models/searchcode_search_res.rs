use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchcodeSearchRes {
    #[serde(rename = "page")]
    pub page: i32,

    #[serde(rename = "limit")]
    pub limit: i32,

    #[serde(rename = "count")]
    pub count: i32,

    #[serde(rename = "searchtype")]
    pub searchtype: String,
    #[serde(rename = "addresses")]
    pub addresses: Vec<models::SearchcodeSearchResAddressesInner>,
}

impl SearchcodeSearchRes {
    pub fn new(
        page: i32,
        limit: i32,
        count: i32,
        searchtype: String,
        addresses: Vec<models::SearchcodeSearchResAddressesInner>,
    ) -> SearchcodeSearchRes {
        SearchcodeSearchRes {
            page,
            limit,
            count,
            searchtype,
            addresses,
        }
    }
}
