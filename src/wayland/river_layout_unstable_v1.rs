#[doc = "manage river layout objects\n\nA global factory for river_layout objects."]
pub mod zriver_layout_manager_v1 {
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
        #[doc = "create a river_layout object\n\nThis creates a new river_layout object for the given wl_output.\n\nAll layout related communication is done through this interface.\n\nThe namespace is used by the compositor to decide which river_layout\nobject will receive layout demands for the output. Namespaces must be\nunique per output. The same namespace may occur on other outputs, as\nlong as that river_layout object comes from the same client. If the\nnamespace is found to not obey these uniqueness rules, the server will\nsend a namespace_in_use event on the river_layout object immediately\nafter creation and ignore all requests on the object aside from the\ndestroy request."]
        GetRiverLayout {
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
                name: "get_river_layout",
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
                Request::GetRiverLayout { .. } => 1,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::Destroy => 1,
                Request::GetRiverLayout { .. } => 1,
            }
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                1 => Some(Object::from_interface::<
                    super::zriver_layout_v1::ZriverLayoutV1,
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
                Request::GetRiverLayout { output, namespace } => Message {
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
                Request::GetRiverLayout { output, namespace } => {
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
    pub struct ZriverLayoutManagerV1(Proxy<ZriverLayoutManagerV1>);
    impl AsRef<Proxy<ZriverLayoutManagerV1>> for ZriverLayoutManagerV1 {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<ZriverLayoutManagerV1>> for ZriverLayoutManagerV1 {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            ZriverLayoutManagerV1(value)
        }
    }
    impl From<ZriverLayoutManagerV1> for Proxy<ZriverLayoutManagerV1> {
        #[inline]
        fn from(value: ZriverLayoutManagerV1) -> Self {
            value.0
        }
    }
    impl std::fmt::Debug for ZriverLayoutManagerV1 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(format_args!("{:?}", self.0))
        }
    }
    impl Interface for ZriverLayoutManagerV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zriver_layout_manager_v1";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &zriver_layout_manager_v1_interface }
        }
    }
    impl ZriverLayoutManagerV1 {
        #[doc = "destroy the river_layout_manager object\n\nThis request indicates that the client will not use the\nriver_layout_manager object any more. Objects that have been created\nthrough this instance are not affected.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        pub fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "create a river_layout object\n\nThis creates a new river_layout object for the given wl_output.\n\nAll layout related communication is done through this interface.\n\nThe namespace is used by the compositor to decide which river_layout\nobject will receive layout demands for the output. Namespaces must be\nunique per output. The same namespace may occur on other outputs, as\nlong as that river_layout object comes from the same client. If the\nnamespace is found to not obey these uniqueness rules, the server will\nsend a namespace_in_use event on the river_layout object immediately\nafter creation and ignore all requests on the object aside from the\ndestroy request."]
        pub fn get_river_layout(
            &self,
            output: &super::wl_output::WlOutput,
            namespace: String,
        ) -> Main<super::zriver_layout_v1::ZriverLayoutV1> {
            let msg = Request::GetRiverLayout {
                output: output.clone(),
                namespace: namespace,
            };
            self.0.send(msg, None).unwrap()
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_GET_RIVER_LAYOUT_SINCE: u32 = 1u32;
    static mut zriver_layout_manager_v1_requests_get_river_layout_types: [*const wl_interface; 3] = [
        unsafe { &super::zriver_layout_v1::zriver_layout_v1_interface as *const wl_interface },
        unsafe { &super::wl_output::wl_output_interface as *const wl_interface },
        NULLPTR as *const wl_interface,
    ];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zriver_layout_manager_v1_requests: [wl_message; 2] = [
        wl_message {
            name: b"destroy\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"get_river_layout\0" as *const u8 as *const c_char,
            signature: b"nos\0" as *const u8 as *const c_char,
            types: unsafe { &zriver_layout_manager_v1_requests_get_river_layout_types as *const _ },
        },
    ];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut zriver_layout_manager_v1_interface: wl_interface = wl_interface {
        name: b"zriver_layout_manager_v1\0" as *const u8 as *const c_char,
        version: 1,
        request_count: 2,
        requests: unsafe { &zriver_layout_manager_v1_requests as *const _ },
        event_count: 0,
        events: NULLPTR as *const wl_message,
    };
}
#[doc = "receive and respond to layout demands\n\nThis interface allows clients to receive layout demands from the\ncompositor for a specific output and subsequently propose positions and\ndimensions of individual views."]
pub mod zriver_layout_v1 {
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
        #[doc = "amount of proposed dimensions does not match amount of views in layout"]
        ProposedDimensionMismatch = 0,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                0 => Some(Error::ProposedDimensionMismatch),
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
        #[doc = "destroy the river_layout object\n\nThis request indicates that the client will not use the river_layout\nobject any more.\n\nThis is a destructor, once sent this object cannot be used any longer."]
        Destroy,
        #[doc = "propose view dimensions\n\nThe client may use this request to push a view dimension proposal to the\nlayout.\n\nView dimension proposals apply to the views in order of the view list.\nAs an example, the first push_view_dimensions request will always affect\nthe first view in the list.\n\nA client must propose position and dimensions for the entire set of\nviews. Proposing to many or to few view dimensions is a protocol error.\n\nThis request may be sent before the corresponding view has been\nadvertised.\n\nThe compositor will transpose the dimensions so that 0 aligns with the\norigin of the usable width and height.\n\nThe serial is used to identify the layout demand this request is a\nresponse to."]
        PushViewDimensions {
            serial: u32,
            x: i32,
            y: i32,
            width: u32,
            height: u32,
        },
        #[doc = "commit a layout\n\nThe client may use this request to signal the compositor that it has\nfinished proposing a layout.\n\nSending this request irrevocably marks the layout as finished and the\nlayout demand will no longer be active. The client may not send any\nother responses to it, doing so is a protocol error.\n\nBeware that there are no guarantees that the proposed layout will be\nused, the compositor may silently ignore it.\n\nThe serial is used to identify the layout demand this request is a\nresponse to."]
        Commit { serial: u32 },
        #[doc = "parameters of layout have changed\n\nThe client may use this request to inform the compositor that one or\nmuliple of the parameters it uses to generate layouts have changed.\n\nIf the client is responsible for the current view layout, the compositor\nmay decide to send a new layout demand to update the layout."]
        ParametersChanged,
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
            super::MessageDesc {
                name: "parameters_changed",
                since: 1,
                signature: &[],
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
                Request::ParametersChanged => 3,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::Destroy => 1,
                Request::PushViewDimensions { .. } => 1,
                Request::Commit { .. } => 1,
                Request::ParametersChanged => 1,
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
                Request::ParametersChanged => Message {
                    sender_id: sender_id,
                    opcode: 3,
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
                Request::ParametersChanged => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(3, &mut _args_array)
                }
            }
        }
    }
    #[derive(Debug)]
    #[non_exhaustive]
    pub enum Event {
        #[doc = "namespace already in use\n\nThe requested namespace is already used by another river_layout object.\nAfter receiving this event, the client should destroy the river_layout\nobject. Any other request will be ignored."]
        NamespaceInUse,
        #[doc = "the compositor is in demand of a layout\n\nThe compositor raises this event to inform the client that it requires a\nlayout for a set of views.\n\nThe usable width and height height indicate the space in which the\nclient can safely position views without interfering with desktop\nwidgets such as panels.\n\nThe serial of this event is used to identify subsequent events and\nrequest as belonging to this layout demand. Beware that the client\nmight need to handle multiple layout demands at the same time.\n\nThe server will ignore responses to all but the most recent\nlayout_demand. Thus, clients should only respond to the most recent\nlayout_demand received. If a newer layout_demand is received before the\nclient has finished responding to an old demand, the client should\nabort. Work on the old demand as any further work would be wasted."]
        LayoutDemand {
            view_amount: u32,
            usable_width: u32,
            usable_height: u32,
            tags: u32,
            serial: u32,
        },
        #[doc = "make layout client aware of view\n\nThis event is raised by the compositor after a layout_demand event.\nIt contains additional information about one out of the set of views\nfor which a layout has been demanded.\n\nIt is guaranteed that every view in the layout will be advertised\nexactly once, in the exact order of the view list.\n\nA client not interested in the additional information may ignore this\nevent.\n\nThe serial is the same as that of the layout demand this event belongs\nto."]
        AdvertiseView {
            tags: u32,
            app_id: Option<String>,
            serial: u32,
        },
        #[doc = "all views have been advertised\n\nThe compositor raises this event when it has advertised the entire set\nof views for which a layout has been demanded.\n\nA client not interested in the additional information may ignore this\nevent.\n\nThe serial is the same as that of the layout demand this event belongs\nto."]
        AdvertiseDone { serial: u32 },
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
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Event::NamespaceInUse => 1,
                Event::LayoutDemand { .. } => 1,
                Event::AdvertiseView { .. } => 1,
                Event::AdvertiseDone { .. } => 1,
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
                        view_amount: {
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
                        view_amount: _args[0].u,
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
    pub struct ZriverLayoutV1(Proxy<ZriverLayoutV1>);
    impl AsRef<Proxy<ZriverLayoutV1>> for ZriverLayoutV1 {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<ZriverLayoutV1>> for ZriverLayoutV1 {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            ZriverLayoutV1(value)
        }
    }
    impl From<ZriverLayoutV1> for Proxy<ZriverLayoutV1> {
        #[inline]
        fn from(value: ZriverLayoutV1) -> Self {
            value.0
        }
    }
    impl std::fmt::Debug for ZriverLayoutV1 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(format_args!("{:?}", self.0))
        }
    }
    impl Interface for ZriverLayoutV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zriver_layout_v1";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &zriver_layout_v1_interface }
        }
    }
    impl ZriverLayoutV1 {
        #[doc = "destroy the river_layout object\n\nThis request indicates that the client will not use the river_layout\nobject any more.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        pub fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "propose view dimensions\n\nThe client may use this request to push a view dimension proposal to the\nlayout.\n\nView dimension proposals apply to the views in order of the view list.\nAs an example, the first push_view_dimensions request will always affect\nthe first view in the list.\n\nA client must propose position and dimensions for the entire set of\nviews. Proposing to many or to few view dimensions is a protocol error.\n\nThis request may be sent before the corresponding view has been\nadvertised.\n\nThe compositor will transpose the dimensions so that 0 aligns with the\norigin of the usable width and height.\n\nThe serial is used to identify the layout demand this request is a\nresponse to."]
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
        #[doc = "commit a layout\n\nThe client may use this request to signal the compositor that it has\nfinished proposing a layout.\n\nSending this request irrevocably marks the layout as finished and the\nlayout demand will no longer be active. The client may not send any\nother responses to it, doing so is a protocol error.\n\nBeware that there are no guarantees that the proposed layout will be\nused, the compositor may silently ignore it.\n\nThe serial is used to identify the layout demand this request is a\nresponse to."]
        pub fn commit(&self, serial: u32) -> () {
            let msg = Request::Commit { serial: serial };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "parameters of layout have changed\n\nThe client may use this request to inform the compositor that one or\nmuliple of the parameters it uses to generate layouts have changed.\n\nIf the client is responsible for the current view layout, the compositor\nmay decide to send a new layout demand to update the layout."]
        pub fn parameters_changed(&self) -> () {
            let msg = Request::ParametersChanged;
            self.0.send::<AnonymousObject>(msg, None);
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_PUSH_VIEW_DIMENSIONS_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_COMMIT_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_PARAMETERS_CHANGED_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_NAMESPACE_IN_USE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_LAYOUT_DEMAND_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_ADVERTISE_VIEW_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_ADVERTISE_DONE_SINCE: u32 = 1u32;
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zriver_layout_v1_requests: [wl_message; 4] = [
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
        wl_message {
            name: b"parameters_changed\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
    ];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zriver_layout_v1_events: [wl_message; 4] = [
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
    ];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut zriver_layout_v1_interface: wl_interface = wl_interface {
        name: b"zriver_layout_v1\0" as *const u8 as *const c_char,
        version: 1,
        request_count: 4,
        requests: unsafe { &zriver_layout_v1_requests as *const _ },
        event_count: 4,
        events: unsafe { &zriver_layout_v1_events as *const _ },
    };
}
