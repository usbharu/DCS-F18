use serde::{Deserialize, Serialize};

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
    pub max_length: Option<u16>,
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
