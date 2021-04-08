// mod build;
mod display;
mod options;
mod wayland;

use crate::wayland::{
    river_layout_unstable_v1::zriver_layout_manager_v1::ZriverLayoutManagerV1,
    river_options_unstable_v1::zriver_options_manager_v1::ZriverOptionsManagerV1,
    river_status_unstable_v1::zriver_status_manager_v1::ZriverStatusManagerV1,
};
use display::{Context, Output};
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
                ZriverLayoutManagerV1,
                1,
                |layout_manager: Main<ZriverLayoutManagerV1>, mut context: DispatchData| {
                    context.get::<Context>().unwrap().globals.layout_manager = Some(layout_manager);
                    context.get::<Context>().unwrap().running = true;
                }
            ],
            [
                ZriverStatusManagerV1,
                1,
                |status_manager: Main<ZriverStatusManagerV1>, mut context: DispatchData| {
                    context.get::<Context>().unwrap().globals.status_manager = Some(status_manager);
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
                ZriverOptionsManagerV1,
                1,
                |options_manager: Main<ZriverOptionsManagerV1>, mut context: DispatchData| {
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
            context.outputs[monitor_index].options.debug();
        }
        context.outputs[monitor_index].update();
    }
}

fn help() {
    println!("\nkile --help");
    println!("  This is the list of options kile operates with");
    println!("    view_padding (int): the padding of each window within the layout.");
    println!("    outer_padding (int): the padding of the output.");
    println!("    smart_padding (int): disables padding if there's only one view on the output.");
    println!("    main_index (int): the index of the main frame.");
    println!("    main_factor (fixed): the ratio of the screen dedicated to the main frame.");
    println!("    main_amount (int): the amount of window in the main frame.");
    println!("    layout_output (string): the layout of the frames / how the output is split into different regions.");
    println!("    layout_frames (string): the layout of the windows within the frames.");
    println!(
        "    layout_per_tag (string): the configuration for the layout in each tag, river supports up to 32."
    );
    println!(
        "      format: \"{format}\"\n",
        format = "1:v:hh 4:h:tt 2:h:hth ..."
    );
    println!("kile <flags>");
    println!("  flags");
    println!("    -m | --m | --monitor <int> : sets index of the monitor Kile will be used on.");
    println!(
        "    -d | --d | --debug : displays the content of the Options struct as events occur."
    );
    println!("    -n | --n | --namespace <string> : the string you assign to the layout option so kile can receive events.");
    println!("    -h | --h | --help : shows this help menu.\n");
}
