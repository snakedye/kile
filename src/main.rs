mod display;
mod layout;
mod options;
// mod build;
mod wayland;
// extern crate smithay_client_toolkit;
// extern crate wayland_commons;
// extern crate wayland_client;
// extern crate wayland_scanner;

use display::Context;
use options::Options;
use wayland_client::protocol::wl_output::WlOutput;
use wayland_client::{Display, GlobalManager};
// use smithay_client_toolkit::environment::MultiGlobalHandler;

fn main() {
    // Connect to the server
    let display = Display::connect_to_env().unwrap();

    let mut event_queue = display.create_event_queue();

    let attached_display = (*display).clone().attach(event_queue.token());

    // We use the GlobalManager convenience provided by the crate, it covers
    // most classic use cases and avoids us the trouble to manually implement
    // the registry
    let globals = GlobalManager::new(&attached_display);

    event_queue
        .sync_roundtrip(&mut (), |_, _, _| unreachable!())
        .unwrap();

    let list = globals.list();

    // let context=Context::new()

    println!("{:?}", list);

    // let options=Options::new(&WlOutput.release(),"New");

    // scanner();
}

// fn init_wayland() {}
