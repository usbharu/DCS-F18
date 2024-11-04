use std::{env, path::PathBuf};

use clap::Parser;
use clap_num::maybe_hex;
use dcs_bios_cli::{main_loop, setup_source};
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short,long,num_args=0..,value_parser=maybe_hex::<u16>)]
    address: Option<Vec<u16>>,
    #[arg(short,long,num_args=0..)]
    id: Option<Vec<String>>,
    #[arg(short, long)]
    path: Option<PathBuf>,
    #[arg(short,long)]
    module: Option<Vec<String>>
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
    println!("{:?}", args.path);
    println!("{:?}", args.address);
    println!("{:?}", args.id);

    let source = setup_source().unwrap();
    main_loop(source);
}
