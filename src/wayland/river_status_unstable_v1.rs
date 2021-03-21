#[doc = "manage river status objects\n\nA global factory for objects that receive status information specific\nto river. It could be used to implement, for example, a status bar."]
pub mod zriver_status_manager_v1 {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface, wl_message};
    use super::{
        smallvec, types_null, AnonymousObject, Argument, ArgumentType, Interface, Main, Message,
        MessageDesc, MessageGroup, Object, ObjectMetadata, Proxy, NULLPTR,
    };
    use std::os::raw::c_char;
    #[derive(Debug)]
    #[non_exhaustive]
    pub enum Request {
        #[doc = "destroy the river_status_manager object\n\nThis request indicates that the client will not use the\nriver_status_manager object any more. Objects that have been created\nthrough this instance are not affected.\n\nThis is a destructor, once sent this object cannot be used any longer."]
        Destroy,
        #[doc = "create an output status object\n\nThis creates a new river_output_status object for the given wl_output."]
        GetRiverOutputStatus { output: super::wl_output::WlOutput },
        #[doc = "create a seat status object\n\nThis creates a new river_seat_status object for the given wl_seat."]
        GetRiverSeatStatus { seat: super::wl_seat::WlSeat },
    }
    impl super::MessageGroup for Request {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "destroy",
                since: 1,
                signature: &[],
                destructor: true,
            },
            super::MessageDesc {
                name: "get_river_output_status",
                since: 1,
                signature: &[super::ArgumentType::NewId, super::ArgumentType::Object],
                destructor: false,
            },
            super::MessageDesc {
                name: "get_river_seat_status",
                since: 1,
                signature: &[super::ArgumentType::NewId, super::ArgumentType::Object],
                destructor: false,
            },
        ];
        type Map = super::ProxyMap;
        fn is_destructor(&self) -> bool {
            match *self {
                Request::Destroy => true,
                _ => false,
            }
        }
        fn opcode(&self) -> u16 {
            match *self {
                Request::Destroy => 0,
                Request::GetRiverOutputStatus { .. } => 1,
                Request::GetRiverSeatStatus { .. } => 2,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::Destroy => 1,
                Request::GetRiverOutputStatus { .. } => 1,
                Request::GetRiverSeatStatus { .. } => 1,
            }
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                1 => Some(Object::from_interface::<
                    super::zriver_output_status_v1::ZriverOutputStatusV1,
                >(version, meta.child())),
                2 => Some(Object::from_interface::<
                    super::zriver_seat_status_v1::ZriverSeatStatusV1,
                >(version, meta.child())),
                _ => None,
            }
        }
        fn from_raw(msg: Message, map: &mut Self::Map) -> Result<Self, ()> {
            panic!("Request::from_raw can not be used Client-side.")
        }
        fn into_raw(self, sender_id: u32) -> Message {
            match self {
                Request::Destroy => Message {
                    sender_id: sender_id,
                    opcode: 0,
                    args: smallvec![],
                },
                Request::GetRiverOutputStatus { output } => Message {
                    sender_id: sender_id,
                    opcode: 1,
                    args: smallvec![Argument::NewId(0), Argument::Object(output.as_ref().id()),],
                },
                Request::GetRiverSeatStatus { seat } => Message {
                    sender_id: sender_id,
                    opcode: 2,
                    args: smallvec![Argument::NewId(0), Argument::Object(seat.as_ref().id()),],
                },
            }
        }
        unsafe fn from_raw_c(
            obj: *mut ::std::os::raw::c_void,
            opcode: u32,
            args: *const wl_argument,
        ) -> Result<Request, ()> {
            panic!("Request::from_raw_c can not be used Client-side.")
        }
        fn as_raw_c_in<F, T>(self, f: F) -> T
        where
            F: FnOnce(u32, &mut [wl_argument]) -> T,
        {
            match self {
                Request::Destroy => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(0, &mut _args_array)
                }
                Request::GetRiverOutputStatus { output } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = ::std::ptr::null_mut() as *mut _;
                    _args_array[1].o = output.as_ref().c_ptr() as *mut _;
                    f(1, &mut _args_array)
                }
                Request::GetRiverSeatStatus { seat } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = ::std::ptr::null_mut() as *mut _;
                    _args_array[1].o = seat.as_ref().c_ptr() as *mut _;
                    f(2, &mut _args_array)
                }
            }
        }
    }
    #[derive(Debug)]
    #[non_exhaustive]
    pub enum Event {}
    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[];
        type Map = super::ProxyMap;
        fn is_destructor(&self) -> bool {
            match *self {}
        }
        fn opcode(&self) -> u16 {
            match *self {}
        }
        fn since(&self) -> u32 {
            match *self {}
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                _ => None,
            }
        }
        fn from_raw(msg: Message, map: &mut Self::Map) -> Result<Self, ()> {
            match msg.opcode {
                _ => Err(()),
            }
        }
        fn into_raw(self, sender_id: u32) -> Message {
            panic!("Event::into_raw can not be used Client-side.")
        }
        unsafe fn from_raw_c(
            obj: *mut ::std::os::raw::c_void,
            opcode: u32,
            args: *const wl_argument,
        ) -> Result<Event, ()> {
            match opcode {
                _ => return Err(()),
            }
        }
        fn as_raw_c_in<F, T>(self, f: F) -> T
        where
            F: FnOnce(u32, &mut [wl_argument]) -> T,
        {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }
    #[derive(Clone, Eq, PartialEq)]
    pub struct ZriverStatusManagerV1(Proxy<ZriverStatusManagerV1>);
    impl AsRef<Proxy<ZriverStatusManagerV1>> for ZriverStatusManagerV1 {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<ZriverStatusManagerV1>> for ZriverStatusManagerV1 {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            ZriverStatusManagerV1(value)
        }
    }
    impl From<ZriverStatusManagerV1> for Proxy<ZriverStatusManagerV1> {
        #[inline]
        fn from(value: ZriverStatusManagerV1) -> Self {
            value.0
        }
    }
    impl std::fmt::Debug for ZriverStatusManagerV1 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(format_args!("{:?}", self.0))
        }
    }
    impl Interface for ZriverStatusManagerV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zriver_status_manager_v1";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &zriver_status_manager_v1_interface }
        }
    }
    impl ZriverStatusManagerV1 {
        #[doc = "destroy the river_status_manager object\n\nThis request indicates that the client will not use the\nriver_status_manager object any more. Objects that have been created\nthrough this instance are not affected.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        pub fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "create an output status object\n\nThis creates a new river_output_status object for the given wl_output."]
        pub fn get_river_output_status(
            &self,
            output: &super::wl_output::WlOutput,
        ) -> Main<super::zriver_output_status_v1::ZriverOutputStatusV1> {
            let msg = Request::GetRiverOutputStatus {
                output: output.clone(),
            };
            self.0.send(msg, None).unwrap()
        }
        #[doc = "create a seat status object\n\nThis creates a new river_seat_status object for the given wl_seat."]
        pub fn get_river_seat_status(
            &self,
            seat: &super::wl_seat::WlSeat,
        ) -> Main<super::zriver_seat_status_v1::ZriverSeatStatusV1> {
            let msg = Request::GetRiverSeatStatus { seat: seat.clone() };
            self.0.send(msg, None).unwrap()
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_GET_RIVER_OUTPUT_STATUS_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_GET_RIVER_SEAT_STATUS_SINCE: u32 = 1u32;
    static mut zriver_status_manager_v1_requests_get_river_output_status_types:
        [*const wl_interface; 2] = [
        unsafe {
            &super::zriver_output_status_v1::zriver_output_status_v1_interface
                as *const wl_interface
        },
        unsafe { &super::wl_output::wl_output_interface as *const wl_interface },
    ];
    static mut zriver_status_manager_v1_requests_get_river_seat_status_types:
        [*const wl_interface; 2] = [
        unsafe {
            &super::zriver_seat_status_v1::zriver_seat_status_v1_interface as *const wl_interface
        },
        unsafe { &super::wl_seat::wl_seat_interface as *const wl_interface },
    ];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zriver_status_manager_v1_requests: [wl_message; 3] = [
        wl_message {
            name: b"destroy\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"get_river_output_status\0" as *const u8 as *const c_char,
            signature: b"no\0" as *const u8 as *const c_char,
            types: unsafe {
                &zriver_status_manager_v1_requests_get_river_output_status_types as *const _
            },
        },
        wl_message {
            name: b"get_river_seat_status\0" as *const u8 as *const c_char,
            signature: b"no\0" as *const u8 as *const c_char,
            types: unsafe {
                &zriver_status_manager_v1_requests_get_river_seat_status_types as *const _
            },
        },
    ];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut zriver_status_manager_v1_interface: wl_interface = wl_interface {
        name: b"zriver_status_manager_v1\0" as *const u8 as *const c_char,
        version: 1,
        request_count: 3,
        requests: unsafe { &zriver_status_manager_v1_requests as *const _ },
        event_count: 0,
        events: NULLPTR as *const wl_message,
    };
}
#[doc = "track output tags and focus\n\nThis interface allows clients to receive information about the current\nwindowing state of an output."]
pub mod zriver_output_status_v1 {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface, wl_message};
    use super::{
        smallvec, types_null, AnonymousObject, Argument, ArgumentType, Interface, Main, Message,
        MessageDesc, MessageGroup, Object, ObjectMetadata, Proxy, NULLPTR,
    };
    use std::os::raw::c_char;
    #[derive(Debug)]
    #[non_exhaustive]
    pub enum Request {
        #[doc = "destroy the river_output_status object\n\nThis request indicates that the client will not use the\nriver_output_status object any more.\n\nThis is a destructor, once sent this object cannot be used any longer."]
        Destroy,
    }
    impl super::MessageGroup for Request {
        const MESSAGES: &'static [super::MessageDesc] = &[super::MessageDesc {
            name: "destroy",
            since: 1,
            signature: &[],
            destructor: true,
        }];
        type Map = super::ProxyMap;
        fn is_destructor(&self) -> bool {
            match *self {
                Request::Destroy => true,
            }
        }
        fn opcode(&self) -> u16 {
            match *self {
                Request::Destroy => 0,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::Destroy => 1,
            }
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                _ => None,
            }
        }
        fn from_raw(msg: Message, map: &mut Self::Map) -> Result<Self, ()> {
            panic!("Request::from_raw can not be used Client-side.")
        }
        fn into_raw(self, sender_id: u32) -> Message {
            match self {
                Request::Destroy => Message {
                    sender_id: sender_id,
                    opcode: 0,
                    args: smallvec![],
                },
            }
        }
        unsafe fn from_raw_c(
            obj: *mut ::std::os::raw::c_void,
            opcode: u32,
            args: *const wl_argument,
        ) -> Result<Request, ()> {
            panic!("Request::from_raw_c can not be used Client-side.")
        }
        fn as_raw_c_in<F, T>(self, f: F) -> T
        where
            F: FnOnce(u32, &mut [wl_argument]) -> T,
        {
            match self {
                Request::Destroy => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(0, &mut _args_array)
                }
            }
        }
    }
    #[derive(Debug)]
    #[non_exhaustive]
    pub enum Event {
        #[doc = "focused tags of the output\n\nSent once binding the interface and again whenever the tag focus of\nthe output changes."]
        FocusedTags { tags: u32 },
        #[doc = "tag state of an output's views\n\nSent once on binding the interface and again whenever the tag state\nof the output changes."]
        ViewTags { tags: Vec<u8> },
    }
    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "focused_tags",
                since: 1,
                signature: &[super::ArgumentType::Uint],
                destructor: false,
            },
            super::MessageDesc {
                name: "view_tags",
                since: 1,
                signature: &[super::ArgumentType::Array],
                destructor: false,
            },
        ];
        type Map = super::ProxyMap;
        fn is_destructor(&self) -> bool {
            match *self {
                _ => false,
            }
        }
        fn opcode(&self) -> u16 {
            match *self {
                Event::FocusedTags { .. } => 0,
                Event::ViewTags { .. } => 1,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Event::FocusedTags { .. } => 1,
                Event::ViewTags { .. } => 1,
            }
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                _ => None,
            }
        }
        fn from_raw(msg: Message, map: &mut Self::Map) -> Result<Self, ()> {
            match msg.opcode {
                0 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::FocusedTags {
                        tags: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                1 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::ViewTags {
                        tags: {
                            if let Some(Argument::Array(val)) = args.next() {
                                *val
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                _ => Err(()),
            }
        }
        fn into_raw(self, sender_id: u32) -> Message {
            panic!("Event::into_raw can not be used Client-side.")
        }
        unsafe fn from_raw_c(
            obj: *mut ::std::os::raw::c_void,
            opcode: u32,
            args: *const wl_argument,
        ) -> Result<Event, ()> {
            match opcode {
                0 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::FocusedTags { tags: _args[0].u })
                }
                1 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::ViewTags {
                        tags: {
                            let array = &*_args[0].a;
                            ::std::slice::from_raw_parts(array.data as *const u8, array.size)
                                .to_owned()
                        },
                    })
                }
                _ => return Err(()),
            }
        }
        fn as_raw_c_in<F, T>(self, f: F) -> T
        where
            F: FnOnce(u32, &mut [wl_argument]) -> T,
        {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }
    #[derive(Clone, Eq, PartialEq)]
    pub struct ZriverOutputStatusV1(Proxy<ZriverOutputStatusV1>);
    impl AsRef<Proxy<ZriverOutputStatusV1>> for ZriverOutputStatusV1 {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<ZriverOutputStatusV1>> for ZriverOutputStatusV1 {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            ZriverOutputStatusV1(value)
        }
    }
    impl From<ZriverOutputStatusV1> for Proxy<ZriverOutputStatusV1> {
        #[inline]
        fn from(value: ZriverOutputStatusV1) -> Self {
            value.0
        }
    }
    impl std::fmt::Debug for ZriverOutputStatusV1 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(format_args!("{:?}", self.0))
        }
    }
    impl Interface for ZriverOutputStatusV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zriver_output_status_v1";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &zriver_output_status_v1_interface }
        }
    }
    impl ZriverOutputStatusV1 {
        #[doc = "destroy the river_output_status object\n\nThis request indicates that the client will not use the\nriver_output_status object any more.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        pub fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.0.send::<AnonymousObject>(msg, None);
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_FOCUSED_TAGS_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_VIEW_TAGS_SINCE: u32 = 1u32;
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zriver_output_status_v1_requests: [wl_message; 1] = [wl_message {
        name: b"destroy\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    }];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zriver_output_status_v1_events: [wl_message; 2] = [
        wl_message {
            name: b"focused_tags\0" as *const u8 as *const c_char,
            signature: b"u\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"view_tags\0" as *const u8 as *const c_char,
            signature: b"a\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
    ];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut zriver_output_status_v1_interface: wl_interface = wl_interface {
        name: b"zriver_output_status_v1\0" as *const u8 as *const c_char,
        version: 1,
        request_count: 1,
        requests: unsafe { &zriver_output_status_v1_requests as *const _ },
        event_count: 2,
        events: unsafe { &zriver_output_status_v1_events as *const _ },
    };
}
#[doc = "track seat focus\n\nThis interface allows clients to receive information about the current\nfocus of a seat. Note that (un)focused_output events will only be sent\nif the client has bound the relevant wl_output globals."]
pub mod zriver_seat_status_v1 {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface, wl_message};
    use super::{
        smallvec, types_null, AnonymousObject, Argument, ArgumentType, Interface, Main, Message,
        MessageDesc, MessageGroup, Object, ObjectMetadata, Proxy, NULLPTR,
    };
    use std::os::raw::c_char;
    #[derive(Debug)]
    #[non_exhaustive]
    pub enum Request {
        #[doc = "destroy the river_seat_status object\n\nThis request indicates that the client will not use the\nriver_seat_status object any more.\n\nThis is a destructor, once sent this object cannot be used any longer."]
        Destroy,
    }
    impl super::MessageGroup for Request {
        const MESSAGES: &'static [super::MessageDesc] = &[super::MessageDesc {
            name: "destroy",
            since: 1,
            signature: &[],
            destructor: true,
        }];
        type Map = super::ProxyMap;
        fn is_destructor(&self) -> bool {
            match *self {
                Request::Destroy => true,
            }
        }
        fn opcode(&self) -> u16 {
            match *self {
                Request::Destroy => 0,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::Destroy => 1,
            }
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                _ => None,
            }
        }
        fn from_raw(msg: Message, map: &mut Self::Map) -> Result<Self, ()> {
            panic!("Request::from_raw can not be used Client-side.")
        }
        fn into_raw(self, sender_id: u32) -> Message {
            match self {
                Request::Destroy => Message {
                    sender_id: sender_id,
                    opcode: 0,
                    args: smallvec![],
                },
            }
        }
        unsafe fn from_raw_c(
            obj: *mut ::std::os::raw::c_void,
            opcode: u32,
            args: *const wl_argument,
        ) -> Result<Request, ()> {
            panic!("Request::from_raw_c can not be used Client-side.")
        }
        fn as_raw_c_in<F, T>(self, f: F) -> T
        where
            F: FnOnce(u32, &mut [wl_argument]) -> T,
        {
            match self {
                Request::Destroy => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(0, &mut _args_array)
                }
            }
        }
    }
    #[derive(Debug)]
    #[non_exhaustive]
    pub enum Event {
        #[doc = "the seat focused an output\n\nSent on binding the interface and again whenever an output gains focus."]
        FocusedOutput { output: super::wl_output::WlOutput },
        #[doc = "the seat unfocused an output\n\nSent whenever an output loses focus."]
        UnfocusedOutput { output: super::wl_output::WlOutput },
        #[doc = "information on the focused view\n\nSent once on binding the interface and again whenever the focused\nview or a property thereof changes. The title may be an empty string\nif no view is focused or the focused view did not set a title."]
        FocusedView { title: String },
    }
    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "focused_output",
                since: 1,
                signature: &[super::ArgumentType::Object],
                destructor: false,
            },
            super::MessageDesc {
                name: "unfocused_output",
                since: 1,
                signature: &[super::ArgumentType::Object],
                destructor: false,
            },
            super::MessageDesc {
                name: "focused_view",
                since: 1,
                signature: &[super::ArgumentType::Str],
                destructor: false,
            },
        ];
        type Map = super::ProxyMap;
        fn is_destructor(&self) -> bool {
            match *self {
                _ => false,
            }
        }
        fn opcode(&self) -> u16 {
            match *self {
                Event::FocusedOutput { .. } => 0,
                Event::UnfocusedOutput { .. } => 1,
                Event::FocusedView { .. } => 2,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Event::FocusedOutput { .. } => 1,
                Event::UnfocusedOutput { .. } => 1,
                Event::FocusedView { .. } => 1,
            }
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                _ => None,
            }
        }
        fn from_raw(msg: Message, map: &mut Self::Map) -> Result<Self, ()> {
            match msg.opcode {
                0 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::FocusedOutput {
                        output: {
                            if let Some(Argument::Object(val)) = args.next() {
                                map.get_or_dead(val).into()
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                1 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::UnfocusedOutput {
                        output: {
                            if let Some(Argument::Object(val)) = args.next() {
                                map.get_or_dead(val).into()
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                2 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::FocusedView {
                        title: {
                            if let Some(Argument::Str(val)) = args.next() {
                                let s = String::from_utf8(val.into_bytes()).unwrap_or_else(|e| {
                                    String::from_utf8_lossy(&e.into_bytes()).into()
                                });
                                s
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                _ => Err(()),
            }
        }
        fn into_raw(self, sender_id: u32) -> Message {
            panic!("Event::into_raw can not be used Client-side.")
        }
        unsafe fn from_raw_c(
            obj: *mut ::std::os::raw::c_void,
            opcode: u32,
            args: *const wl_argument,
        ) -> Result<Event, ()> {
            match opcode {
                0 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::FocusedOutput {
                        output: Proxy::<super::wl_output::WlOutput>::from_c_ptr(
                            _args[0].o as *mut _,
                        )
                        .into(),
                    })
                }
                1 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::UnfocusedOutput {
                        output: Proxy::<super::wl_output::WlOutput>::from_c_ptr(
                            _args[0].o as *mut _,
                        )
                        .into(),
                    })
                }
                2 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::FocusedView {
                        title: ::std::ffi::CStr::from_ptr(_args[0].s)
                            .to_string_lossy()
                            .into_owned(),
                    })
                }
                _ => return Err(()),
            }
        }
        fn as_raw_c_in<F, T>(self, f: F) -> T
        where
            F: FnOnce(u32, &mut [wl_argument]) -> T,
        {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }
    #[derive(Clone, Eq, PartialEq)]
    pub struct ZriverSeatStatusV1(Proxy<ZriverSeatStatusV1>);
    impl AsRef<Proxy<ZriverSeatStatusV1>> for ZriverSeatStatusV1 {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<ZriverSeatStatusV1>> for ZriverSeatStatusV1 {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            ZriverSeatStatusV1(value)
        }
    }
    impl From<ZriverSeatStatusV1> for Proxy<ZriverSeatStatusV1> {
        #[inline]
        fn from(value: ZriverSeatStatusV1) -> Self {
            value.0
        }
    }
    impl std::fmt::Debug for ZriverSeatStatusV1 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(format_args!("{:?}", self.0))
        }
    }
    impl Interface for ZriverSeatStatusV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zriver_seat_status_v1";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &zriver_seat_status_v1_interface }
        }
    }
    impl ZriverSeatStatusV1 {
        #[doc = "destroy the river_seat_status object\n\nThis request indicates that the client will not use the\nriver_seat_status object any more.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        pub fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.0.send::<AnonymousObject>(msg, None);
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_FOCUSED_OUTPUT_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_UNFOCUSED_OUTPUT_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_FOCUSED_VIEW_SINCE: u32 = 1u32;
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zriver_seat_status_v1_requests: [wl_message; 1] = [wl_message {
        name: b"destroy\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    }];
    static mut zriver_seat_status_v1_events_focused_output_types: [*const wl_interface; 1] =
        [unsafe { &super::wl_output::wl_output_interface as *const wl_interface }];
    static mut zriver_seat_status_v1_events_unfocused_output_types: [*const wl_interface; 1] =
        [unsafe { &super::wl_output::wl_output_interface as *const wl_interface }];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zriver_seat_status_v1_events: [wl_message; 3] = [
        wl_message {
            name: b"focused_output\0" as *const u8 as *const c_char,
            signature: b"o\0" as *const u8 as *const c_char,
            types: unsafe { &zriver_seat_status_v1_events_focused_output_types as *const _ },
        },
        wl_message {
            name: b"unfocused_output\0" as *const u8 as *const c_char,
            signature: b"o\0" as *const u8 as *const c_char,
            types: unsafe { &zriver_seat_status_v1_events_unfocused_output_types as *const _ },
        },
        wl_message {
            name: b"focused_view\0" as *const u8 as *const c_char,
            signature: b"s\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
    ];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut zriver_seat_status_v1_interface: wl_interface = wl_interface {
        name: b"zriver_seat_status_v1\0" as *const u8 as *const c_char,
        version: 1,
        request_count: 1,
        requests: unsafe { &zriver_seat_status_v1_requests as *const _ },
        event_count: 3,
        events: unsafe { &zriver_seat_status_v1_events as *const _ },
    };
}
