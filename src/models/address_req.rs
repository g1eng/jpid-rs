use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddressReq {
    #[serde(rename = "pref_code", skip_serializing_if = "Option::is_none")]
    pub pref_code: Option<String>,

    #[serde(rename = "pref_name", skip_serializing_if = "Option::is_none")]
    pub pref_name: Option<String>,

    #[serde(rename = "pref_kana", skip_serializing_if = "Option::is_none")]
    pub pref_kana: Option<String>,

    #[serde(rename = "pref_roma", skip_serializing_if = "Option::is_none")]
    pub pref_roma: Option<String>,

    #[serde(rename = "city_code", skip_serializing_if = "Option::is_none")]
    pub city_code: Option<String>,

    #[serde(rename = "city_name", skip_serializing_if = "Option::is_none")]
    pub city_name: Option<String>,

    #[serde(rename = "city_kana", skip_serializing_if = "Option::is_none")]
    pub city_kana: Option<String>,

    #[serde(rename = "city_roma", skip_serializing_if = "Option::is_none")]
    pub city_roma: Option<String>,

    #[serde(rename = "town_name", skip_serializing_if = "Option::is_none")]
    pub town_name: Option<String>,

    #[serde(rename = "town_kana", skip_serializing_if = "Option::is_none")]
    pub town_kana: Option<String>,

    #[serde(rename = "town_roma", skip_serializing_if = "Option::is_none")]
    pub town_roma: Option<String>,

    #[serde(rename = "freeword", skip_serializing_if = "Option::is_none")]
    pub freeword: Option<String>,

    #[serde(rename = "flg_getcity", skip_serializing_if = "Option::is_none")]
    pub flg_getcity: Option<f64>,

    #[serde(rename = "flg_getpref", skip_serializing_if = "Option::is_none")]
    pub flg_getpref: Option<f64>,

    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,

    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

impl AddressReq {
    pub fn new() -> AddressReq {
        AddressReq {
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
            freeword: None,
            flg_getcity: None,
            flg_getpref: None,
            page: None,
            limit: None,
        }
    }
}
