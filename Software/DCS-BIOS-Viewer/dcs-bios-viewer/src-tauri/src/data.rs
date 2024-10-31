use dcs_bios_const_generator::Function;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum Data {
    IntegerData { address: u16, value: u16 },
    StringData { address: u16, value: String },
}
#[derive(Deserialize, Serialize, Debug)]
pub struct Module<'a> {
    pub name: &'a str,
    pub functions: HashMap<&'a str, Function>,
}
