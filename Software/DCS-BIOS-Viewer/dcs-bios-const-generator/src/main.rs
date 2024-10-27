use dcs_bios_const_generator::{parse_file, write_file};
use getopts::Options;
use std::env;
use std::fs::File;
use std::io::Error;


fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    let program = args[0].clone();
    let mut options = Options::new();
    options.optopt("o", "output", "output file name", "INPUT");
    options.optflag("h", "help", "print this help menu");
    options.optflag("v", "version", "print version");
    options.optmulti("i", "input", "input files", "OUTPUT");

    let matches = options.parse(&args[1..]).expect("Failed to parse arguments");

    if matches.opt_present("v") {
        print_help(options,program);
        return Ok(());
    }

    if matches.opt_count("i") == 0 {
        print_help(options,program);
        return Ok(());
    }

    if matches.opt_present("h") {
        print_help(options,program);
        return Ok(());
    }

    let mut vec = Vec::new();

    for x in matches.opt_strs("i") {
        let file = File::open(x)?;
        vec.append(&mut parse_file(file)?);
    }

    let output = File::create(matches.opt_get_default("o", "dcs_bios_const.rs".to_string()).unwrap_or("dcs_bios_const.rs".to_string()))?;

    write_file(output, &vec)
}

fn print_help(options: Options,program:String) {
    println!("DCS-BIOS Const Generator Version: {}", env!("CARGO_PKG_VERSION"));
    let brief = format!("{} -i <INPUT> [-i <INPUT>]... [-o <OUTPUT>] [-h] [-v]", program);
    println!("{}", options.usage(&brief)); 
}