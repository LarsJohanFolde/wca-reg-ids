#![allow(unused)]
use serde_json::Value;

use crate::country_names;

#[derive(Debug)]
pub struct Person {
    pub id: String,
    pub wca_id: String,
    pub name: String,
    pub country_id: String,
    pub is_competing: bool,
}

impl Person {
    pub fn new(
        id_as_string: String,
        name: String,
        wca_id: String,
        country_id: String,
        is_competing: bool,
    ) -> Self {
        Self {
            id: id_as_string,
            name,
            wca_id,
            country_id,
            is_competing,
        }
    }

    pub fn from(data: &Value) -> Option<Self> {
        Some(Self {
            id: data["registrantId"].to_string(),
            name: data["name"].to_string().replace("\"", ""),
            wca_id: data["wcaId"].to_string().replace("\"", ""),
            country_id: country_names::common_name(data["countryIso2"].as_str().unwrap()).to_string(),
            is_competing: data["registration"]["isCompeting"].as_bool().unwrap_or(false),
        })
    }
}
