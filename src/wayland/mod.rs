extern crate wayland_client;
extern crate wayland_commons;

// Re-export only the actual code, and then only use this re-export
// The `generated` module below is just some boilerplate to properly isolate stuff
// and avoid exposing internal details.
//
// You can use all the types from my_protocol as if they went from `wayland_client::protocol`.
pub use wayland::client as river_layout_v2;

pub mod wayland {
    // The generated code tends to trigger a lot of warnings
    // so we isolate it into a very permissive module
    #![allow(dead_code, non_camel_case_types, unused_unsafe, unused_variables)]
    #![allow(non_upper_case_globals, non_snake_case, unused_imports)]

    pub mod client {
        // These imports are used by the generated code
        pub(crate) use wayland_client::protocol::wl_output;
        pub(crate) use wayland_client::{protocol, sys};
        pub(crate) use wayland_client::{
            AnonymousObject, Attached, Display, GlobalManager, Main, Proxy, ProxyMap,
        };
        pub(crate) use wayland_commons::map::{Object, ObjectMetadata};
        pub(crate) use wayland_commons::smallvec;
        pub(crate) use wayland_commons::wire::{Argument, ArgumentType, Message, MessageDesc};
        pub(crate) use wayland_commons::{Interface, MessageGroup};
        // If you protocol interacts with objects from other protocols, you'll need to import
        // their modules, like so:
        pub(crate) use wayland_client::protocol::{wl_region, wl_seat, wl_surface};
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/wayland/river_layout_v2.rs"
        ));
    }
}
