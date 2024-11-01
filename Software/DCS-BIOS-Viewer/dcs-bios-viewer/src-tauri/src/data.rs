use dcs_bios_const_generator::Function;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Data {
    #[serde(rename = "integer")]
    IntegerData {  address: u16, value: u16 },
    #[serde(rename = "string")]
    StringData { address: u16, value: String },
}
#[derive(Deserialize, Serialize, Debug)]
pub struct Module<'a> {
    pub name: &'a str,
    pub functions: HashMap<&'a str, Function>,
}

impl Data {
    pub fn address(&self) -> u16 {
        match self {
            Data::IntegerData { address, value: _ } => *address ,
            Data::StringData { address, value: _ } => *address ,
        }
    }
}