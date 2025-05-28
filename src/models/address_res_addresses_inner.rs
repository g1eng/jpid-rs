use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddressResAddressesInner {
    #[serde(rename = "zip_code", skip_serializing_if = "Option::is_none")]
    pub zip_code: Option<String>,
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
}

impl AddressResAddressesInner {
    pub fn new() -> AddressResAddressesInner {
        AddressResAddressesInner {
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
        }
    }
}
