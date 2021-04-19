extern crate wayland_scanner;

use wayland_scanner::{Side, generate_code};
use std::path::Path;

pub fn scanner() {
    generate("river_layout_v1");
    generate("river_options_v2");
}

fn generate(protocol_name:&str) {
    let out_dir = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/wayland/"));

    let mut protocol_dir = String::from(concat!(env!("CARGO_MANIFEST_DIR"), "/protocols/"));
    protocol_dir.push_str(protocol_name);
    protocol_dir.push_str(".xml");
    protocol_dir = protocol_dir.replace("_","-");

    let protocol = Path::new(&protocol_dir);
    let mut protocol_file = protocol_name.to_string();
    protocol_file.push_str(".rs");

    generate_code(
        protocol,
        out_dir.join(protocol_file),
        Side::Client,
    );
}
