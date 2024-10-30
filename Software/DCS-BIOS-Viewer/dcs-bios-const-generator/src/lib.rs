use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Type {
    integer,
    string,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Output {
    pub address: u16,
    pub description: String,
    pub mask: Option<u16>,
    pub max_value: Option<u16>,
    pub shift_by: Option<u16>,
    pub suffix: String,
    pub r#type: Type,
    pub max_length: Option<u16>
}
#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Interface {
    set_state,
    action,
    fixed_step,
    variable_step,
    set_string,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Input {
    description: String,
    interface: Interface,
    arguments: Option<String>,
    max_value: Option<u16>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MomentaryPositions {
    none,
    first_and_last,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ControlType {
    selector,
    limited_dial,
    analog_gauge,
    display,
    led,
    toggle_switch,
    frequency,
    emergency_parking_brake,
    metadata,
    mission_computer_switch,
    analog_dial,
    fixed_step_dial,
    #[serde(other)]
    other,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Function {
    pub api_variant: Option<String>,
    pub category: String,
    pub control_type: ControlType,
    pub description: String,
    pub identifier: String,
    pub inputs: Vec<Input>,
    pub momentary_positions: Option<MomentaryPositions>,
    pub outputs: Vec<Output>,
}

pub fn parse_file(file: File) -> Result<Vec<Function>, std::io::Error> {
    let result: HashMap<String, HashMap<String, Function>> = serde_json::from_reader(file)?;
    let vec = result
        .into_values()
        .flat_map(|x| x.into_values())
        .collect::<Vec<Function>>();

    Ok(vec)
}

pub fn write_file(file: File, vec: &Vec<Function>) -> Result<(), std::io::Error> {
    let mut writer = BufWriter::new(file);

    writer.write_all(format!("pub struct Address<'a>{{\n    pub id:&'a str,\n    pub address:u16,\n    pub mask:u16,\n    pub shift:u16\n}}\n").as_bytes())?;

    for x in vec {
        if x.outputs.len() == 1 {
            let output = x.outputs.first().unwrap();
            writer.write_all(format!("pub const {}: Address = Address{{\n    id: \"{}\",\n    address: {},\n    mask: {},\n    shift: {}\n}};\n", x.identifier, x.identifier, output.address, output.mask.unwrap_or(0), output.shift_by.unwrap_or(0)).as_bytes())?
        }
    }

    writer.flush()?;
    Ok(())
}
