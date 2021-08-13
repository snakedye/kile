mod client;
mod layout;
mod lexer;
mod wayland;

use std::env;
use client::{Globals, Output};
use wayland_client::protocol::wl_output::WlOutput;
use wayland_client::{Display, GlobalManager, Main};
use crate::wayland::river_layout_v3::river_layout_manager_v3::RiverLayoutManagerV3;

fn main() {
    let mut args = env::args();
    let mut namespace = String::from("kile");
    args.next();
    loop {
        match args.next() {
            Some(flag) => match flag.as_str() {
                "--namespace" | "--n" | "-n" => {
                    if let Some(name) = args.next() {
                        namespace = name;
                    }
                }
                "--help" | "-h" | "--h" => {
                    print!("Usage: kile [option]\n\n");
                    print!("  -n, --n, --namespace <string> : the namespace of kile.\n");
                    std::process::exit(0);
                }
                _ => break,
            },
            None => break,
        }
    }

    let mut globals = Globals::new(namespace);
    let display = Display::connect_to_env().unwrap();
    let mut event_queue = display.create_event_queue();
    let attached_display = (*display).clone().attach(event_queue.token());

    GlobalManager::new_with_cb(
        &attached_display,
        wayland_client::global_filter!(
            [
                RiverLayoutManagerV3,
                1,
                |layout_manager: Main<RiverLayoutManagerV3>, mut globals: DispatchData| {
                    globals.get::<Globals>().unwrap().layout_manager = Some(layout_manager);
                }
            ],
            [
                WlOutput,
                3,
                |output: Main<WlOutput>, mut globals: DispatchData| {
                    output.quick_assign(move |_, _, _| {});
                    let output = Output::new(output);
                    if let Some(globals) = globals.get::<Globals>() {
                        output.layout_filter(globals.layout_manager.as_ref(), globals.namespace.clone());
                    }
                }
            ]
        ),
    );

    loop {
        event_queue
            .dispatch(&mut globals, |event, object, _| {
                panic!(
                    "[callop] Encountered an orphan event: {}@{}: {}",
                    event.interface,
                    object.as_ref().id(),
                    event.name
                );
            })
            .unwrap();
    }
}
