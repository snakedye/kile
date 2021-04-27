use std::os::raw::{c_char, c_void};
const NULLPTR: *const c_void = 0 as *const c_void;
static mut types_null: [*const sys::common::wl_interface; 5] = [
    NULLPTR as *const sys::common::wl_interface,
    NULLPTR as *const sys::common::wl_interface,
    NULLPTR as *const sys::common::wl_interface,
    NULLPTR as *const sys::common::wl_interface,
    NULLPTR as *const sys::common::wl_interface,
];
#[doc = "manage river layout objects\n\nA global factory for river_layout_v2 objects."]
pub mod river_layout_manager_v2 {
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
        #[doc = "destroy the river_layout_manager object\n\nThis request indicates that the client will not use the\nriver_layout_manager object any more. Objects that have been created\nthrough this instance are not affected.\n\nThis is a destructor, once sent this object cannot be used any longer."]
        Destroy,
        #[doc = "create a river_layout_v2 object\n\nThis creates a new river_layout_v2 object for the given wl_output.\n\nAll layout related communication is done through this interface.\n\nThe namespace is used by the compositor to decide which river_layout_v2\nobject will receive layout demands for the output.\n\nThe namespace is required to be be unique per-output. Furthermore,\ntwo separate clients may not share a namespace on separate outputs. If\nthese conditions are not upheld, the the namespace_in_use event will\nbe sent directly after creation of the river_layout_v2 object."]
        GetLayout {
            output: super::wl_output::WlOutput,
            namespace: String,
        },
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
                name: "get_layout",
                since: 1,
                signature: &[
                    super::ArgumentType::NewId,
                    super::ArgumentType::Object,
                    super::ArgumentType::Str,
                ],
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
                Request::GetLayout { .. } => 1,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::Destroy => 1,
                Request::GetLayout { .. } => 1,
            }
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                1 => Some(Object::from_interface::<
                    super::river_layout_v2::RiverLayoutV2,
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
                Request::GetLayout { output, namespace } => Message {
                    sender_id: sender_id,
                    opcode: 1,
                    args: smallvec![
                        Argument::NewId(0),
                        Argument::Object(output.as_ref().id()),
                        Argument::Str(Box::new(unsafe {
                            ::std::ffi::CString::from_vec_unchecked(namespace.into())
                        })),
                    ],
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
                Request::GetLayout { output, namespace } => {
                    let mut _args_array: [wl_argument; 3] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = ::std::ptr::null_mut() as *mut _;
                    _args_array[1].o = output.as_ref().c_ptr() as *mut _;
                    let _arg_2 = ::std::ffi::CString::new(namespace).unwrap();
                    _args_array[2].s = _arg_2.as_ptr();
                    f(1, &mut _args_array)
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
    pub struct RiverLayoutManagerV2(Proxy<RiverLayoutManagerV2>);
    impl AsRef<Proxy<RiverLayoutManagerV2>> for RiverLayoutManagerV2 {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<RiverLayoutManagerV2>> for RiverLayoutManagerV2 {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            RiverLayoutManagerV2(value)
        }
    }
    impl From<RiverLayoutManagerV2> for Proxy<RiverLayoutManagerV2> {
        #[inline]
        fn from(value: RiverLayoutManagerV2) -> Self {
            value.0
        }
    }
    impl std::fmt::Debug for RiverLayoutManagerV2 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(format_args!("{:?}", self.0))
        }
    }
    impl Interface for RiverLayoutManagerV2 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "river_layout_manager_v2";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &river_layout_manager_v2_interface }
        }
    }
    impl RiverLayoutManagerV2 {
        #[doc = "destroy the river_layout_manager object\n\nThis request indicates that the client will not use the\nriver_layout_manager object any more. Objects that have been created\nthrough this instance are not affected.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        pub fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "create a river_layout_v2 object\n\nThis creates a new river_layout_v2 object for the given wl_output.\n\nAll layout related communication is done through this interface.\n\nThe namespace is used by the compositor to decide which river_layout_v2\nobject will receive layout demands for the output.\n\nThe namespace is required to be be unique per-output. Furthermore,\ntwo separate clients may not share a namespace on separate outputs. If\nthese conditions are not upheld, the the namespace_in_use event will\nbe sent directly after creation of the river_layout_v2 object."]
        pub fn get_layout(
            &self,
            output: &super::wl_output::WlOutput,
            namespace: String,
        ) -> Main<super::river_layout_v2::RiverLayoutV2> {
            let msg = Request::GetLayout {
                output: output.clone(),
                namespace: namespace,
            };
            self.0.send(msg, None).unwrap()
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_GET_LAYOUT_SINCE: u32 = 1u32;
    static mut river_layout_manager_v2_requests_get_layout_types: [*const wl_interface; 3] = [
        unsafe { &super::river_layout_v2::river_layout_v2_interface as *const wl_interface },
        unsafe { &super::wl_output::wl_output_interface as *const wl_interface },
        NULLPTR as *const wl_interface,
    ];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut river_layout_manager_v2_requests: [wl_message; 2] = [
        wl_message {
            name: b"destroy\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"get_layout\0" as *const u8 as *const c_char,
            signature: b"nos\0" as *const u8 as *const c_char,
            types: unsafe { &river_layout_manager_v2_requests_get_layout_types as *const _ },
        },
    ];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut river_layout_manager_v2_interface: wl_interface = wl_interface {
        name: b"river_layout_manager_v2\0" as *const u8 as *const c_char,
        version: 1,
        request_count: 2,
        requests: unsafe { &river_layout_manager_v2_requests as *const _ },
        event_count: 0,
        events: NULLPTR as *const wl_message,
    };
}
#[doc = "receive and respond to layout demands\n\nThis interface allows clients to receive layout demands from the\ncompositor for a specific output and subsequently propose positions and\ndimensions of individual views."]
pub mod river_layout_v2 {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface, wl_message};
    use super::{
        smallvec, types_null, AnonymousObject, Argument, ArgumentType, Interface, Main, Message,
        MessageDesc, MessageGroup, Object, ObjectMetadata, Proxy, NULLPTR,
    };
    use std::os::raw::c_char;
    #[repr(u32)]
    #[derive(Copy, Clone, Debug, PartialEq)]
    #[non_exhaustive]
    pub enum Error {
        #[doc = "number of proposed dimensions does not match number of views in layout"]
        CountMismatch = 0,
        #[doc = "the layout demand with the provided serial was already committed"]
        AlreadyCommitted = 1,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                0 => Some(Error::CountMismatch),
                1 => Some(Error::AlreadyCommitted),
                _ => Option::None,
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    #[derive(Debug)]
    #[non_exhaustive]
    pub enum Request {
        #[doc = "destroy the river_layout_v2 object\n\nThis request indicates that the client will not use the river_layout_v2\nobject any more.\n\nThis is a destructor, once sent this object cannot be used any longer."]
        Destroy,
        #[doc = "propose dimensions of the next view\n\nThis request proposes a size and position of a view in the layout demand\nwith matching serial.\n\nPushed view dimensions apply to the views in the same order they were\nadvertised. That is, the first push_view_dimensions request applies\nto the first view advertised, the second to the second, and so on.\n\nA client must propose position and dimensions for the entire set of\nviews. Proposing too many or too few view dimensions is a protocol error.\n\nThis request may be sent before the corresponding view has been\nadvertised.\n\nThe x and y coordinates are relative to the usable area of the output,\nwith (0,0) as the top left corner."]
        PushViewDimensions {
            serial: u32,
            x: i32,
            y: i32,
            width: u32,
            height: u32,
        },
        #[doc = "commit a layout\n\nThis request indicates that the client is done pushing dimensions\nand the compositor may apply the layout. This completes the layout\ndemand with matching serial, any other requests sent with the serial\nare a protocol error.\n\nThe compositor is free to use this proposed layout however it chooses,\nincluding ignoring it."]
        Commit { serial: u32 },
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
                name: "push_view_dimensions",
                since: 1,
                signature: &[
                    super::ArgumentType::Uint,
                    super::ArgumentType::Int,
                    super::ArgumentType::Int,
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                ],
                destructor: false,
            },
            super::MessageDesc {
                name: "commit",
                since: 1,
                signature: &[super::ArgumentType::Uint],
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
                Request::PushViewDimensions { .. } => 1,
                Request::Commit { .. } => 2,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::Destroy => 1,
                Request::PushViewDimensions { .. } => 1,
                Request::Commit { .. } => 1,
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
                Request::PushViewDimensions {
                    serial,
                    x,
                    y,
                    width,
                    height,
                } => Message {
                    sender_id: sender_id,
                    opcode: 1,
                    args: smallvec![
                        Argument::Uint(serial),
                        Argument::Int(x),
                        Argument::Int(y),
                        Argument::Uint(width),
                        Argument::Uint(height),
                    ],
                },
                Request::Commit { serial } => Message {
                    sender_id: sender_id,
                    opcode: 2,
                    args: smallvec![Argument::Uint(serial),],
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
                Request::PushViewDimensions {
                    serial,
                    x,
                    y,
                    width,
                    height,
                } => {
                    let mut _args_array: [wl_argument; 5] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].u = serial;
                    _args_array[1].i = x;
                    _args_array[2].i = y;
                    _args_array[3].u = width;
                    _args_array[4].u = height;
                    f(1, &mut _args_array)
                }
                Request::Commit { serial } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].u = serial;
                    f(2, &mut _args_array)
                }
            }
        }
    }
    #[derive(Debug)]
    #[non_exhaustive]
    pub enum Event {
        #[doc = "the requested namespace is already in use\n\nAfter this event is sent, all requests aside from the destroy event\nwill be ignored by the server. If the client wishes to try again with\na different namespace they must create a new river_layout_v2 object."]
        NamespaceInUse,
        #[doc = "the compositor requires a layout\n\nThe compositor sends this event to inform the client that it requires a\nlayout for a set of views.\n\nThe usable width and height height indicate the space in which the\nclient can safely position views without interfering with desktop\nwidgets such as panels.\n\nThe serial of this event is used to identify subsequent events and\nrequest as belonging to this layout demand. Beware that the client\nmight need to handle multiple layout demands at the same time.\n\nThe server will ignore responses to all but the most recent\nlayout demand. Thus, clients are only required to respond to the most\nrecent layout_demand received. If a newer layout_demand is received\nbefore the client has finished responding to an old demand, the client\nmay abort work on the old demand as any further work would be wasted."]
        LayoutDemand {
            view_count: u32,
            usable_width: u32,
            usable_height: u32,
            tags: u32,
            serial: u32,
        },
        #[doc = "make layout client aware of view\n\nThis event is sent by the server as part of the layout demand with\nmatching serial. It provides additional information about one of\nthe views to be arranged.\n\nEvery view part of the layout demand is advertised exactly once,\nin the order of the view list."]
        AdvertiseView {
            tags: u32,
            app_id: Option<String>,
            serial: u32,
        },
        #[doc = "all views have been advertised\n\nThis event is sent by the server as the last event of the layout\ndemand with matching serial, after all advertise_view events."]
        AdvertiseDone { serial: u32 },
        #[doc = "an int value has been set\n\nThis event indicates that the value of this river_layout_v2 object\nwith the given name has been set to the given value.\n\nThis event will be followed by a layout_demand if necessary (i.e. if\nthis layout object is currently being used by the compositor to\nlayout an output)"]
        SetIntValue { name: String, value: i32 },
        #[doc = "an int value has been modified\n\nThis event indicates that the value of this river_layout_v2 object\nwith the given name has been modifed by the given delta.\n\nThis event will be followed by a layout_demand if necessary (i.e. if\nthis layout object is currently being used by the compositor to\nlayout an output)"]
        ModIntValue { name: String, delta: i32 },
        #[doc = "a fixed value has been set\n\nThis event indicates that the value of this river_layout_v2 object\nwith the given name has been set to the given value.\n\nThis event will be followed by a layout_demand if necessary (i.e. if\nthis layout object is currently being used by the compositor to\nlayout an output)"]
        SetFixedValue { name: String, value: f64 },
        #[doc = "a fixed value has been modified\n\nThis event indicates that the value of this river_layout_v2 object\nwith the given name has been modifed by the given delta.\n\nThis event will be followed by a layout_demand if necessary (i.e. if\nthis layout object is currently being used by the compositor to\nlayout an output)"]
        ModFixedValue { name: String, delta: f64 },
        #[doc = "a string value has been set\n\nThis event indicates that the value of this river_layout_v2 object\nwith the given name has been set to the given value.\n\nThis event will be followed by a layout_demand if necessary (i.e. if\nthis layout object is currently being used by the compositor to\nlayout an output)"]
        SetStringValue { name: String, value: String },
    }
    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "namespace_in_use",
                since: 1,
                signature: &[],
                destructor: false,
            },
            super::MessageDesc {
                name: "layout_demand",
                since: 1,
                signature: &[
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                ],
                destructor: false,
            },
            super::MessageDesc {
                name: "advertise_view",
                since: 1,
                signature: &[
                    super::ArgumentType::Uint,
                    super::ArgumentType::Str,
                    super::ArgumentType::Uint,
                ],
                destructor: false,
            },
            super::MessageDesc {
                name: "advertise_done",
                since: 1,
                signature: &[super::ArgumentType::Uint],
                destructor: false,
            },
            super::MessageDesc {
                name: "set_int_value",
                since: 1,
                signature: &[super::ArgumentType::Str, super::ArgumentType::Int],
                destructor: false,
            },
            super::MessageDesc {
                name: "mod_int_value",
                since: 1,
                signature: &[super::ArgumentType::Str, super::ArgumentType::Int],
                destructor: false,
            },
            super::MessageDesc {
                name: "set_fixed_value",
                since: 1,
                signature: &[super::ArgumentType::Str, super::ArgumentType::Fixed],
                destructor: false,
            },
            super::MessageDesc {
                name: "mod_fixed_value",
                since: 1,
                signature: &[super::ArgumentType::Str, super::ArgumentType::Fixed],
                destructor: false,
            },
            super::MessageDesc {
                name: "set_string_value",
                since: 1,
                signature: &[super::ArgumentType::Str, super::ArgumentType::Str],
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
                Event::NamespaceInUse => 0,
                Event::LayoutDemand { .. } => 1,
                Event::AdvertiseView { .. } => 2,
                Event::AdvertiseDone { .. } => 3,
                Event::SetIntValue { .. } => 4,
                Event::ModIntValue { .. } => 5,
                Event::SetFixedValue { .. } => 6,
                Event::ModFixedValue { .. } => 7,
                Event::SetStringValue { .. } => 8,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Event::NamespaceInUse => 1,
                Event::LayoutDemand { .. } => 1,
                Event::AdvertiseView { .. } => 1,
                Event::AdvertiseDone { .. } => 1,
                Event::SetIntValue { .. } => 1,
                Event::ModIntValue { .. } => 1,
                Event::SetFixedValue { .. } => 1,
                Event::ModFixedValue { .. } => 1,
                Event::SetStringValue { .. } => 1,
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
                0 => Ok(Event::NamespaceInUse),
                1 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::LayoutDemand {
                        view_count: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        usable_width: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        usable_height: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        tags: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        serial: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                2 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::AdvertiseView {
                        tags: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        app_id: {
                            if let Some(Argument::Str(val)) = args.next() {
                                let s = String::from_utf8(val.into_bytes()).unwrap_or_else(|e| {
                                    String::from_utf8_lossy(&e.into_bytes()).into()
                                });
                                if s.len() == 0 {
                                    None
                                } else {
                                    Some(s)
                                }
                            } else {
                                return Err(());
                            }
                        },
                        serial: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                3 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::AdvertiseDone {
                        serial: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                4 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::SetIntValue {
                        name: {
                            if let Some(Argument::Str(val)) = args.next() {
                                let s = String::from_utf8(val.into_bytes()).unwrap_or_else(|e| {
                                    String::from_utf8_lossy(&e.into_bytes()).into()
                                });
                                s
                            } else {
                                return Err(());
                            }
                        },
                        value: {
                            if let Some(Argument::Int(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                5 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::ModIntValue {
                        name: {
                            if let Some(Argument::Str(val)) = args.next() {
                                let s = String::from_utf8(val.into_bytes()).unwrap_or_else(|e| {
                                    String::from_utf8_lossy(&e.into_bytes()).into()
                                });
                                s
                            } else {
                                return Err(());
                            }
                        },
                        delta: {
                            if let Some(Argument::Int(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                6 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::SetFixedValue {
                        name: {
                            if let Some(Argument::Str(val)) = args.next() {
                                let s = String::from_utf8(val.into_bytes()).unwrap_or_else(|e| {
                                    String::from_utf8_lossy(&e.into_bytes()).into()
                                });
                                s
                            } else {
                                return Err(());
                            }
                        },
                        value: {
                            if let Some(Argument::Fixed(val)) = args.next() {
                                (val as f64) / 256.
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                7 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::ModFixedValue {
                        name: {
                            if let Some(Argument::Str(val)) = args.next() {
                                let s = String::from_utf8(val.into_bytes()).unwrap_or_else(|e| {
                                    String::from_utf8_lossy(&e.into_bytes()).into()
                                });
                                s
                            } else {
                                return Err(());
                            }
                        },
                        delta: {
                            if let Some(Argument::Fixed(val)) = args.next() {
                                (val as f64) / 256.
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                8 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::SetStringValue {
                        name: {
                            if let Some(Argument::Str(val)) = args.next() {
                                let s = String::from_utf8(val.into_bytes()).unwrap_or_else(|e| {
                                    String::from_utf8_lossy(&e.into_bytes()).into()
                                });
                                s
                            } else {
                                return Err(());
                            }
                        },
                        value: {
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
                0 => Ok(Event::NamespaceInUse),
                1 => {
                    let _args = ::std::slice::from_raw_parts(args, 5);
                    Ok(Event::LayoutDemand {
                        view_count: _args[0].u,
                        usable_width: _args[1].u,
                        usable_height: _args[2].u,
                        tags: _args[3].u,
                        serial: _args[4].u,
                    })
                }
                2 => {
                    let _args = ::std::slice::from_raw_parts(args, 3);
                    Ok(Event::AdvertiseView {
                        tags: _args[0].u,
                        app_id: if _args[1].s.is_null() {
                            None
                        } else {
                            Some(
                                ::std::ffi::CStr::from_ptr(_args[1].s)
                                    .to_string_lossy()
                                    .into_owned(),
                            )
                        },
                        serial: _args[2].u,
                    })
                }
                3 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::AdvertiseDone { serial: _args[0].u })
                }
                4 => {
                    let _args = ::std::slice::from_raw_parts(args, 2);
                    Ok(Event::SetIntValue {
                        name: ::std::ffi::CStr::from_ptr(_args[0].s)
                            .to_string_lossy()
                            .into_owned(),
                        value: _args[1].i,
                    })
                }
                5 => {
                    let _args = ::std::slice::from_raw_parts(args, 2);
                    Ok(Event::ModIntValue {
                        name: ::std::ffi::CStr::from_ptr(_args[0].s)
                            .to_string_lossy()
                            .into_owned(),
                        delta: _args[1].i,
                    })
                }
                6 => {
                    let _args = ::std::slice::from_raw_parts(args, 2);
                    Ok(Event::SetFixedValue {
                        name: ::std::ffi::CStr::from_ptr(_args[0].s)
                            .to_string_lossy()
                            .into_owned(),
                        value: (_args[1].f as f64) / 256.,
                    })
                }
                7 => {
                    let _args = ::std::slice::from_raw_parts(args, 2);
                    Ok(Event::ModFixedValue {
                        name: ::std::ffi::CStr::from_ptr(_args[0].s)
                            .to_string_lossy()
                            .into_owned(),
                        delta: (_args[1].f as f64) / 256.,
                    })
                }
                8 => {
                    let _args = ::std::slice::from_raw_parts(args, 2);
                    Ok(Event::SetStringValue {
                        name: ::std::ffi::CStr::from_ptr(_args[0].s)
                            .to_string_lossy()
                            .into_owned(),
                        value: ::std::ffi::CStr::from_ptr(_args[1].s)
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
    pub struct RiverLayoutV2(Proxy<RiverLayoutV2>);
    impl AsRef<Proxy<RiverLayoutV2>> for RiverLayoutV2 {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<RiverLayoutV2>> for RiverLayoutV2 {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            RiverLayoutV2(value)
        }
    }
    impl From<RiverLayoutV2> for Proxy<RiverLayoutV2> {
        #[inline]
        fn from(value: RiverLayoutV2) -> Self {
            value.0
        }
    }
    impl std::fmt::Debug for RiverLayoutV2 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(format_args!("{:?}", self.0))
        }
    }
    impl Interface for RiverLayoutV2 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "river_layout_v2";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &river_layout_v2_interface }
        }
    }
    impl RiverLayoutV2 {
        #[doc = "destroy the river_layout_v2 object\n\nThis request indicates that the client will not use the river_layout_v2\nobject any more.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        pub fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "propose dimensions of the next view\n\nThis request proposes a size and position of a view in the layout demand\nwith matching serial.\n\nPushed view dimensions apply to the views in the same order they were\nadvertised. That is, the first push_view_dimensions request applies\nto the first view advertised, the second to the second, and so on.\n\nA client must propose position and dimensions for the entire set of\nviews. Proposing too many or too few view dimensions is a protocol error.\n\nThis request may be sent before the corresponding view has been\nadvertised.\n\nThe x and y coordinates are relative to the usable area of the output,\nwith (0,0) as the top left corner."]
        pub fn push_view_dimensions(
            &self,
            serial: u32,
            x: i32,
            y: i32,
            width: u32,
            height: u32,
        ) -> () {
            let msg = Request::PushViewDimensions {
                serial: serial,
                x: x,
                y: y,
                width: width,
                height: height,
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "commit a layout\n\nThis request indicates that the client is done pushing dimensions\nand the compositor may apply the layout. This completes the layout\ndemand with matching serial, any other requests sent with the serial\nare a protocol error.\n\nThe compositor is free to use this proposed layout however it chooses,\nincluding ignoring it."]
        pub fn commit(&self, serial: u32) -> () {
            let msg = Request::Commit { serial: serial };
            self.0.send::<AnonymousObject>(msg, None);
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_PUSH_VIEW_DIMENSIONS_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_COMMIT_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_NAMESPACE_IN_USE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_LAYOUT_DEMAND_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_ADVERTISE_VIEW_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_ADVERTISE_DONE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_SET_INT_VALUE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_MOD_INT_VALUE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_SET_FIXED_VALUE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_MOD_FIXED_VALUE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_SET_STRING_VALUE_SINCE: u32 = 1u32;
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut river_layout_v2_requests: [wl_message; 3] = [
        wl_message {
            name: b"destroy\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"push_view_dimensions\0" as *const u8 as *const c_char,
            signature: b"uiiuu\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"commit\0" as *const u8 as *const c_char,
            signature: b"u\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
    ];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut river_layout_v2_events: [wl_message; 9] = [
        wl_message {
            name: b"namespace_in_use\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"layout_demand\0" as *const u8 as *const c_char,
            signature: b"uuuuu\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"advertise_view\0" as *const u8 as *const c_char,
            signature: b"u?su\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"advertise_done\0" as *const u8 as *const c_char,
            signature: b"u\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"set_int_value\0" as *const u8 as *const c_char,
            signature: b"si\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"mod_int_value\0" as *const u8 as *const c_char,
            signature: b"si\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"set_fixed_value\0" as *const u8 as *const c_char,
            signature: b"sf\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"mod_fixed_value\0" as *const u8 as *const c_char,
            signature: b"sf\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"set_string_value\0" as *const u8 as *const c_char,
            signature: b"ss\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
    ];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut river_layout_v2_interface: wl_interface = wl_interface {
        name: b"river_layout_v2\0" as *const u8 as *const c_char,
        version: 1,
        request_count: 3,
        requests: unsafe { &river_layout_v2_requests as *const _ },
        event_count: 9,
        events: unsafe { &river_layout_v2_events as *const _ },
    };
}
