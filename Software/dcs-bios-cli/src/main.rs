use dcs_bios_cli::{main_loop, setup_source};

fn main() {
    let source = setup_source().unwrap();
    main_loop(source);
}
