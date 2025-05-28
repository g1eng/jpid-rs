use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchcodeSearchResAddressesInner {
    #[serde(
        rename = "dgacode",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub dgacode: Option<Option<String>>,
    #[serde(rename = "zip_code", skip_serializing_if = "Option::is_none")]
    pub zip_code: Option<i32>,
    #[serde(rename = "pref_code", skip_serializing_if = "Option::is_none")]
    pub pref_code: Option<String>,
    #[serde(rename = "pref_name", skip_serializing_if = "Option::is_none")]
    pub pref_name: Option<String>,
    #[serde(
        rename = "pref_kana",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub pref_kana: Option<Option<String>>,
    #[serde(
        rename = "pref_roma",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub pref_roma: Option<Option<String>>,
    #[serde(rename = "city_code", skip_serializing_if = "Option::is_none")]
    pub city_code: Option<i32>,
    #[serde(rename = "city_name", skip_serializing_if = "Option::is_none")]
    pub city_name: Option<String>,
    #[serde(
        rename = "city_kana",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub city_kana: Option<Option<String>>,
    #[serde(
        rename = "city_roma",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub city_roma: Option<Option<String>>,
    #[serde(rename = "town_name", skip_serializing_if = "Option::is_none")]
    pub town_name: Option<String>,
    #[serde(
        rename = "town_kana",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub town_kana: Option<Option<String>>,
    #[serde(
        rename = "town_roma",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub town_roma: Option<Option<String>>,
    #[serde(rename = "biz_name", skip_serializing_if = "Option::is_none")]
    pub biz_name: Option<String>,
    #[serde(rename = "biz_kana", skip_serializing_if = "Option::is_none")]
    pub biz_kana: Option<String>,
    #[serde(
        rename = "biz_roma",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub biz_roma: Option<Option<String>>,
    #[serde(rename = "block_name", skip_serializing_if = "Option::is_none")]
    pub block_name: Option<String>,
    #[serde(
        rename = "other_name",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub other_name: Option<Option<String>>,
    #[serde(
        rename = "address",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub address: Option<Option<String>>,
    #[serde(
        rename = "longitude",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub longitude: Option<Option<f64>>,
    #[serde(
        rename = "latitude",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub latitude: Option<Option<f64>>,
}

impl SearchcodeSearchResAddressesInner {
    pub fn new() -> SearchcodeSearchResAddressesInner {
        SearchcodeSearchResAddressesInner {
            dgacode: None,
            zip_code: None,
            pref_code: None,
            pref_name: None,
            pref_kana: None,
            pref_roma: None,
            city_code: None,
            city_name: None,
            city_kana: None,
            city_roma: None,
            town_name: None,
            town_kana: None,
            town_roma: None,
            biz_name: None,
            biz_kana: None,
            biz_roma: None,
            block_name: None,
            other_name: None,
            address: None,
            longitude: None,
            latitude: None,
        }
    }
}
