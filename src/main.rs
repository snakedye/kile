// mod build;
mod display;
mod options;
mod wayland;

use crate::wayland::{
    river_layout_unstable_v1::zriver_layout_manager_v1::ZriverLayoutManagerV1,
    river_options_unstable_v1::zriver_options_manager_v1::ZriverOptionsManagerV1,
    river_status_unstable_v1::zriver_status_manager_v1::ZriverStatusManagerV1,
};
use display::Context;
use wayland_client::protocol::{
    wl_output::WlOutput,
    wl_seat::WlSeat,
};
use wayland_client::Main;
use wayland_client::{Display, GlobalManager};
use std::env;

fn main() {
    let display = Display::connect_to_env().unwrap();

    let mut event_queue = display.create_event_queue();

    let attached_display = (*display).clone().attach(event_queue.token());

    let args: Vec<String> = env::args().collect();
    let namespace = if args.len() > 1 {
        args[1].clone()
    } else { String::from("kile") };

    let mut context = Context::new(namespace);

    let _globals = GlobalManager::new_with_cb(
        &attached_display,
        wayland_client::global_filter!(
            [
                ZriverLayoutManagerV1,
                1,
                |layout_manager: Main<ZriverLayoutManagerV1>, mut context: DispatchData| {
                    context.get::<Context>().unwrap().layout_manager = Some(layout_manager);
                    context.get::<Context>().unwrap().running = true;
                }
            ],
            [
                ZriverStatusManagerV1,
                1,
                |status_manager: Main<ZriverStatusManagerV1>, mut context: DispatchData| {
                    context.get::<Context>().unwrap().status_manager = Some(status_manager);
                }
            ],
            [
                ZriverOptionsManagerV1,
                1,
                |options_manager: Main<ZriverOptionsManagerV1>, mut context: DispatchData| {
                    context.get::<Context>().unwrap().options_manager = Some(options_manager);
                }
            ],
            [
                WlOutput,
                3,
                |output: Main<WlOutput>, mut context: DispatchData| {
                    output.quick_assign(move |_, _, _| {});
                    context.get::<Context>().unwrap().output = Some(output.detach());
                }
            ],
            [
                WlSeat,
                3,
                |seat: Main<WlSeat>, mut context: DispatchData| {
                    seat.quick_assign(move |_, _, _| {});
                    context.get::<Context>().unwrap().seat = Some(seat.detach());
                }
            ]
        ),
    );

    event_queue
        .sync_roundtrip(&mut context, |_, _, _| unreachable!())
        .unwrap();

    context.init();

    while context.running {
        event_queue
            .dispatch(&mut context.options, |event, object, _| {
                panic!(
                    "[callop] Encountered an orphan event: {}@{}: {}",
                    event.interface,
                    object.as_ref().id(),
                    event.name
                );
            })
            .unwrap();
        context.update();
    }
}
