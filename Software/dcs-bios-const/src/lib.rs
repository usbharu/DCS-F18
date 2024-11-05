use std::{
    collections::HashMap,
    fs::File,
    io::{BufWriter, Error, Write},
};

use json_type::{Function, Type};

mod json_type;

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

pub fn write_file_enum(file: File, vec: &Vec<Function>) -> Result<(), Error> {
    let mut writer = BufWriter::new(file);

    writer.write_all(format!("pub enum Output<'a>{{\nStringType {{ address: u16,\nsuffix: &'a str,\nmax_length: u16\n}},\nIntegerType{{ address:u16,\nmask:u16,\nshift_by:u16}}}}\n").as_bytes())?;

    for ele in vec {
        for (index, output) in ele.outputs.iter().enumerate() {
            let bytes = match output.r#type {
                            Type::integer => format!("pub const {}_OUTPUT_{}: Output = IntegerType{{\naddress: {},\nmask: {},\nshift_by: {}}};\n",ele.identifier,index,output.address,output.mask.unwrap_or(65535),output.shift_by.unwrap_or(0)),
                            Type::string => format!("pub const {}_OUTPUT_{}: Output = StringType{{\naddress: {},\nmax_length: {},\nsuffix: \"{}\",\n}};\n",ele.identifier,index,output.address,output.max_length.unwrap_or(2),output.suffix),
                        };
            writer.write_all(bytes.as_bytes())?;
        }
    }
    writer.flush()?;
    Ok(())
}
