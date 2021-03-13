mod frame;
mod engine;
mod options;
extern crate wayland_scanner;


use crate::frame::Frame;
use crate::wayland::Option;
use crate::wayland::Request;
use std::env::var;
use std::path::Path;
use wayland_scanner::{Side, generate_code};

pub fn main() {

    // Location of the xml file, relative to the `Cargo.toml`
    let layout_protocol="./protocols/river-layout-unstable-v1.xml";
    let options_protocol="./protocols/river-options-unstable-v1.xml";

    // Target directory for the generate files
    let out_dir=let out_dir = Path::new("./wayland");;

    generate_code(
        layout_protocol,
        out_dir.join("river_layout_unstable.rs"),
        Side::Client, // Replace by `Side::Server` for server-side code
    );

    generate_code(
        options_protocol,
        out_dir.join("river_options_unstable.rs"),
        Side::Client, // Replace by `Side::Server` for server-side code
    );

    let options=Option::new().set_option();
    let output=Frame::new(options.get_options()).generate();
    output.generate();
}

