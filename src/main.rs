// mod build;
mod client;
mod options;
mod wayland;

use crate::wayland::{
    river_layout_v1::river_layout_manager_v1::RiverLayoutManagerV1,
    river_options_v2::river_options_manager_v2::RiverOptionsManagerV2,
};
use client::{Context, Output};
use std::env;
use wayland_client::protocol::wl_output::WlOutput;
use wayland_client::{Display, GlobalManager, Main};

fn main() {
    let display = Display::connect_to_env().unwrap();

    let mut event_queue = display.create_event_queue();

    let attached_display = (*display).clone().attach(event_queue.token());

    let mut context = Context::new();

    GlobalManager::new_with_cb(
        &attached_display,
        wayland_client::global_filter!(
            [
                RiverLayoutManagerV1,
                1,
                |layout_manager: Main<RiverLayoutManagerV1>, mut context: DispatchData| {
                    context.get::<Context>().unwrap().globals.layout_manager = Some(layout_manager);
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
            ],
            [
                RiverOptionsManagerV2,
                1,
                |options_manager: Main<RiverOptionsManagerV2>, mut context: DispatchData| {
                    context.get::<Context>().unwrap().globals.options_manager =
                        Some(options_manager);
                }
            ]
        ),
    );

    event_queue
        .sync_roundtrip(&mut context, |_, _, _| unreachable!())
        .unwrap();

    let mut args = env::args();
    let mut debug = false;
    let mut monitor_index = 0;
    args.next();
    loop {
        match args.next() {
            Some(flag) => match flag.as_str() {
                "--debug" | "--d" | "-d" => debug = true,
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
        event_queue
            .dispatch(&mut context.outputs[monitor_index], |event, object, _| {
                panic!(
                    "[callop] Encountered an orphan event: {}@{}: {}",
                    event.interface,
                    object.as_ref().id(),
                    event.name
                );
            })
            .unwrap();
        if debug {
            context.outputs[monitor_index].debug();
        }
        context.outputs[monitor_index].update();
    }
}

fn help() {
    println!("Usage: kile [option]\n");
    println!("  -m | --m | --monitor <int> : sets index of the monitor Kile will be used on.");
    println!("  -d | --d | --debug : displays the content of the Options struct as events occur.");
    println!("  -n | --n | --namespace <string> : the string you assign to the layout option so kile can receive events.");
    println!("  -h | --h | --help : shows this help message.");
}
