mod build;
mod parser;
mod client;
mod options;
mod wayland;

use crate::wayland::{
    river_layout_v2::river_layout_manager_v2::RiverLayoutManagerV2,
};
use client::{Context, Output};
use std::env;
use wayland_client::protocol::wl_output::WlOutput;
use wayland_client::{Display, GlobalManager, Main};

fn main() {

    build::main();

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
                    context.get::<Context>().unwrap().running = true;
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

    let mut args = env::args();
    let mut monitor_index = 0;
    args.next();
    loop {
        match args.next() {
            Some(flag) => match flag.as_str() {
                "--namespace" | "--n" | "-n" => {
                    context.namespace = args.next().unwrap_or(String::from("kile"))
                }
                "--monitor" | "--m" | "-m" => {
                    match args.next().unwrap_or(String::from("0")).parse::<usize>() {
                        Ok(index) => {
                            monitor_index = if index >= context.outputs.len() {
                                0
                            } else {
                                index
                            }
                        }
                        Err(v) => println!("{}", v),
                    }
                }
                "--help" | "-h" | "--h" => {
                    help();
                    context.running = false;
                }
                _ => break,
            },
            None => break,
        }
    }

    context.init(monitor_index);

    while context.running {
        if let Err(e) = display.flush() {
            if e.kind() != ::std::io::ErrorKind::WouldBlock {
                eprintln!("Error while trying to flush the wayland socket: {:?}", e);
            }
        }
        if let Some(guard) = event_queue.prepare_read() {
            // prepare_read() returns None if there are already events pending in this
            // event queue, in which case there is no need to try to read from the socket
            if let Err(e) = guard.read_events() {
                if e.kind() != ::std::io::ErrorKind::WouldBlock {
                    // if read_events() returns Err(WouldBlock), this just means that no new
                    // messages are available to be read
                    eprintln!("Error while trying to read from the wayland socket: {:?}", e);
                }
            }
        }
        for output in &mut context.outputs {
        event_queue
            .dispatch_pending(output, |event, object, _| {
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

fn help() {
    println!("Usage: kile [option]\n");
    println!("  -m | --m | --monitor <int> : sets index of the monitor Kile will be used on.");
    println!("  -n | --n | --namespace <string> : the string you assign to the layout option so kile can receive events.");
    println!("  -h | --h | --help : shows this help message.");
}
