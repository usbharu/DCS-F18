use dcs_bios::source;
use dcs_bios_cli::{main_loop, setup_source};

fn main() {

    let v = 1..=4;
    println!("{:?} {} {}",v,v.start(),v.end());

    let source = setup_source().unwrap();;
    main_loop(source);
}
