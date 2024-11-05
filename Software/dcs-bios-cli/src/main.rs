use std::fs::File;
use std::{env, path::PathBuf};

use clap::Parser;
use clap_num::maybe_hex;
use dcs_bios_cli::filter::{self, build_filter};
use dcs_bios_cli::{list_modules, main_loop, setup_source};
use dcs_bios_const::parse_file;
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short,long,num_args=0..,value_parser=maybe_hex::<u16>)]
    address: Vec<u16>,
    #[arg(short,long,num_args=0..)]
    id: Vec<String>,
    #[arg(short, long)]
    path: Option<PathBuf>,
    #[arg(short, long)]
    module: Vec<String>,
}

fn main() {
    let mut args = Args::parse();

    if args.path.is_none() {
        let user_profile = env::var("USERPROFILE").ok().map(|v| {
            [
                v,
                "Saved Games".into(),
                "DCS".into(),
                "Scripts".into(),
                "DCS-BIOS".into(),
                "doc".into(),
                "json".into(),
            ]
            .iter()
            .collect()
        });
        args.path = user_profile;
    }

    let modules = list_modules(args.path.unwrap());

    for ele in &modules {
        println!("{}", ele.0);
    }

    let functions = modules
        .iter()
        .filter(|p| args.module.contains(&p.0))
        .filter_map(|f| parse_file(File::open(f.1.path()).unwrap()).ok())
        .flatten()
        .collect();

    let filters = build_filter(args.id, args.address, functions);

    println!("{:?}", filters);

    let source = setup_source().unwrap();
    main_loop(source,filters);
}
