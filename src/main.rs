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
use wayland_client::protocol::{wl_output::WlOutput, wl_seat::WlSeat};
use wayland_client::Main;
use wayland_client::{Display, EventQueue, GlobalManager};

fn main() {
    let display = Display::connect_to_env().unwrap();

    let mut event_queue = display.create_event_queue();

    let attached_display = (*display).clone().attach(event_queue.token());

    let mut debug = false;

    let namespace = {
        let mut args = env::args();
        args.next();
        match args.next() {
            Some(str)=> {
                match str.as_str() {
                    "--help" | "-h" | "--h" => {
                        help();
                        std::process::exit(0);
                    }
                    "--debug" | "--d" | "-d" =>{
                        debug=true;
                        match args.next() {
                            Some(namespace)=>namespace,
                            None=>String::from("kile")
                        }
                    }
                    _=>str,
                }
            }
            None=>String::from("kile"),
        }
    };

    let mut context = Context::new(namespace);

    let _globals = GlobalManager::new_with_cb(
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
                    context.get::<Context>().unwrap().globals.options_manager = Some(options_manager);
                }
            ],
            [
                WlSeat,
                3,
                |seat: Main<WlSeat>, mut context: DispatchData| {
                    seat.quick_assign(move |_, _, _| {});
                    context.get::<Context>().unwrap().globals.seats.push(Some(seat));
                }
            ]
        ),
    );

    event_queue
        .sync_roundtrip(&mut context, |_, _, _| unreachable!())
        .unwrap();

    context.init();

    while context.running {
        for output in &mut context.outputs {
            event_queue
                .dispatch(output, |event, object, _| {
                    panic!(
                        "[callop] Encountered an orphan event: {}@{}: {}",
                        event.interface,
                        object.as_ref().id(),
                        event.name
                    );
                })
                .unwrap();
        }
        context.run();
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
    println!("kile <namespace>");
    println!("  namespace: the string you assign to the layout option so kile can receive events.");
    println!(
        "  By default the namespace is set to \"{kile}\"\n",
        kile = "kile"
    );
    println!("kile --debug <namespace>");
    println!("  shows \"Options\" as events occur\n");
}
