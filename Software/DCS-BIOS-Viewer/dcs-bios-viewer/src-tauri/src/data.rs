use dcs_bios_const_generator::Function;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Data {
    pub(crate) address: u16,
    pub(crate) value: u16,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct Module<'a> {
    pub name: &'a str,
    pub functions: HashMap<&'a str, Function>,
}
