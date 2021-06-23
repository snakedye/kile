mod client;
mod layout;
mod lexer;
mod wayland;

use crate::wayland::river_layout_v2::river_layout_manager_v2::RiverLayoutManagerV2;
use client::{Globals, Output};
use std::env;
use wayland_client::protocol::wl_output::WlOutput;
use wayland_client::{Display, GlobalManager, Main};

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

    let mut globals = Globals::new();
    let display = Display::connect_to_env().unwrap();
    let mut event_queue = display.create_event_queue();
    let attached_display = (*display).clone().attach(event_queue.token());

    GlobalManager::new_with_cb(
        &attached_display,
        wayland_client::global_filter!(
            [
                RiverLayoutManagerV2,
                1,
                |layout_manager: Main<RiverLayoutManagerV2>, mut globals: DispatchData| {
                    globals.get::<Globals>().unwrap().layout_manager = Some(layout_manager);
                }
            ],
            [
                WlOutput,
                3,
                |output: Main<WlOutput>, mut globals: DispatchData| {
                    output.quick_assign(move |_, _, _| {});
                    let output = Output::new(output);
                    globals.get::<Globals>().unwrap().outputs.push(output);
                }
            ]
        ),
    );

    event_queue
        .sync_roundtrip(&mut globals, |_, _, _| unreachable!())
        .unwrap();

    let layout_manager = globals.layout_manager.as_ref();
    for output in globals.outputs {
        output.layout_filter(layout_manager, namespace.clone());
    }

    if layout_manager.is_some() {
        loop {
            event_queue
                .dispatch(&mut (), |event, object, _| {
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
}
