mod client;
mod options;
mod parser;
mod wayland;

use crate::wayland::river_layout_v2::river_layout_manager_v2::RiverLayoutManagerV2;
use client::{Context, Output};
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
                    namespace = args.next().unwrap_or(String::from("kile"))
                }
                "--help" | "-h" | "--h" => {
                    help();
                    std::process::exit(0);
                }
                _ => break,
            },
            None => break,
        }
    }


    let display = Display::connect_to_env().unwrap();

    let mut event_queue = display.create_event_queue();

    let attached_display = (*display).clone().attach(event_queue.token());

    let mut context = Context::new();

    GlobalManager::new_with_cb(
        &attached_display,
        wayland_client::global_filter!(
            [
                RiverLayoutManagerV2,
                1,
                |layout_manager: Main<RiverLayoutManagerV2>, mut context: DispatchData| {
                    context.get::<Context>().unwrap().layout_manager = Some(layout_manager);
                }
            ],
            [
                WlOutput,
                3,
                |output: Main<WlOutput>, mut context: DispatchData| {
                    output.quick_assign(move |_, _, _| {});
                    let output = Output::new(output.detach());
                    context.get::<Context>().unwrap().outputs.push(output);
                }
            ]
        ),
    );

    event_queue
        .sync_roundtrip(&mut context, |_, _, _| unreachable!())
        .unwrap();

    let layout_manager = context.layout_manager.as_ref();
    for output in context.outputs {
        output.layout_filter(layout_manager, namespace.clone());
    }

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

fn help() {
    println!("Usage: kile [option]\n");
    println!("  -m | --m | --monitor <int> : sets index of the monitor Kile will be used on.");
    println!("  -n | --n | --namespace <string> : the string you assign to the layout option so kile can receive events.");
    println!("  -h | --h | --help : shows this help message.");
}
