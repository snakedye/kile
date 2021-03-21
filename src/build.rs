extern crate wayland_scanner;

use wayland_scanner::{Side, generate_code};
use std::env::var;
use std::path::Path;

pub fn scanner() {

    // Location of the xml file, relative to the `Cargo.toml`
    let layout_protocol = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/protocols/river-layout-unstable-v1.xml"
    );
    let options_protocol = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/protocols/river-options-unstable-v1.xml"
    );
    let status_protocol = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/protocols/river-status-unstable-v1.xml"
    );

    // Target directory for the generate files
    let out_dir = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/wayland/"));

    println!("{:?}", out_dir);

    generate_code(
        layout_protocol,
        out_dir.join("river_layout_unstable_v1.rs"),
        Side::Client,
    );

    generate_code(
        options_protocol,
        out_dir.join("river_options_unstable_v1.rs"),
        Side::Client,
    );

    generate_code(
        status_protocol,
        out_dir.join("river_status_unstable_v1.rs"),
        Side::Client,
    );
}

