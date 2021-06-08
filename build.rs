extern crate wayland_scanner;

use std::fs::{File, OpenOptions};
use std::path::Path;
use std::process::{Command, Stdio};
use wayland_scanner::{generate_code, Side};

pub fn main() {
    generate("river_layout_v2");
    match Command::new("scdoc").spawn() {
        Ok(_) => {
            let input = File::open(Path::new("./doc/kile.1.scd")).unwrap();
            let output = OpenOptions::new()
                .write(true)
                .create(true)
                .open(Path::new("./doc/kile.1.gz"))
                .unwrap();
            Command::new("scdoc")
                .stdin(Stdio::from(input))
                .stdout(output)
                .spawn()
                .expect("Failed to execute command");
        }
        Err(_) => {}
    }
}

fn generate(protocol_name: &str) {
    let out_dir = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/wayland/"));

    let mut protocol_dir = String::from(concat!(env!("CARGO_MANIFEST_DIR"), "/protocol/"));
    protocol_dir.push_str(protocol_name);
    protocol_dir.push_str(".xml");
    protocol_dir = protocol_dir.replace("_", "-");

    let protocol = Path::new(&protocol_dir);
    let mut protocol_file = protocol_name.to_string();
    protocol_file.push_str(".rs");

    generate_code(protocol, out_dir.join(protocol_file), Side::Client);
}
