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
use wayland_client::protocol::wl_output::WlOutput;
use crate::wayland::{
    river_layout_unstable_v1::{
        zriver_layout_manager_v1::ZriverLayoutManagerV1,
    },
    river_options_unstable_v1::{
        zriver_options_manager_v1::ZriverOptionsManagerV1,
    },
};
use wayland_client::{
    Display,
    GlobalManager};
use wayland_client::Main;
// use wayland_client::{Display, GlobalManager};
// use smithay_client_toolkit::environment::MultiGlobalHandler;


fn main() {
    // Connect to the server
    let display = Display::connect_to_env().unwrap();

    let mut event_queue = display.create_event_queue();

    let attached_display = (*display).clone().attach(event_queue.token());

    // We use the GlobalManager convenience provided by the crate, it covers
    // most classic use cases and avoids us the trouble to manually implement
    // the registry

    let mut main_context=Context::new(String::from("test"));

    let globals = GlobalManager::new_with_cb(
        &attached_display,
        wayland_client::global_filter!(
            [ZriverLayoutManagerV1, 1,|layout_manager: Main<ZriverLayoutManagerV1>, mut context: DispatchData<>| {
                context.get::<Context>().unwrap().layout_manager=Some(layout_manager);
            }],
            [ZriverOptionsManagerV1, 1,|options_manager: Main<ZriverOptionsManagerV1>,mut  context: DispatchData| {
                context.get::<Context>().unwrap().options_manager=Some(options_manager);
            }],
            [WlOutput, 2,|output: Main<WlOutput>, mut context: DispatchData| {
                context.get::<Context>().unwrap().output=Some(output.detach());
            }]
        )
    );

    event_queue
        .sync_roundtrip(&mut main_context, |_, _, _| unreachable!()).unwrap();

    main_context.init();

    while !main_context.running {
        main_context.update();
        event_queue
            .sync_roundtrip(&mut main_context, |_, _, _| unreachable!())
            .unwrap();
    }
}

