mod display;
mod layout;
mod options;
// mod build;
mod wayland;
// extern crate wayland_commons;
// extern crate wayland_client;
// extern crate wayland_scanner;

use wayland_client::{Display, GlobalManager};

fn main() {
    // Connect to the server
    let display = Display::connect_to_env().unwrap();

    let mut event_queue = display.create_event_queue();

    let attached_display = (*display).clone().attach(event_queue.token());

    // We use the GlobalManager convenience provided by the crate, it covers
    // most classic use cases and avoids us the trouble to manually implement
    // the registry
    let globals = GlobalManager::new(&attached_display);

    // scanner();
}

fn init_wayland() {}
