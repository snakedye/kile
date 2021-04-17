use std::os::raw::{c_char, c_void};
const NULLPTR: *const c_void = 0 as *const c_void;
static mut types_null: [*const sys::common::wl_interface; 1] =
    [NULLPTR as *const sys::common::wl_interface];
#[doc = "set and retrieve options\n\nThis protocol allows clients to access a typed key-value store of\noptions. These options are identified by string keys and are scoped\neither globally or per-output. This protocol does not define any\nsemantic meaning of the options, that is left up to compositors.\n\nCompositors are free to set options themselves at any time, though\nthe type of any given option is immutable once set.\n\nOptions may never be unset once set."]
pub mod zriver_options_manager_v1 {
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
        #[doc = "destroy the zriver_options_manager_v1 object\n\nThis request indicates that the client will not use the manager object\nany more. Objects that have been created through this instance are\nnot affected.\n\nThis is a destructor, once sent this object cannot be used any longer."]
        Destroy,
        #[doc = "get an option handle for the given key\n\nIf the output argument is non-null, the option is local to the given\noutput. Otherwise it is considered global."]
        GetOptionHandle {
            key: String,
            output: Option<super::wl_output::WlOutput>,
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
                name: "get_option_handle",
                since: 1,
                signature: &[
                    super::ArgumentType::Str,
                    super::ArgumentType::Object,
                    super::ArgumentType::NewId,
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
                Request::GetOptionHandle { .. } => 1,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::Destroy => 1,
                Request::GetOptionHandle { .. } => 1,
            }
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                1 => Some(Object::from_interface::<
                    super::zriver_option_handle_v1::ZriverOptionHandleV1,
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
                Request::GetOptionHandle { key, output } => Message {
                    sender_id: sender_id,
                    opcode: 1,
                    args: smallvec![
                        Argument::Str(Box::new(unsafe {
                            ::std::ffi::CString::from_vec_unchecked(key.into())
                        })),
                        Argument::Object(output.map(|o| o.as_ref().id()).unwrap_or(0)),
                        Argument::NewId(0),
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
                Request::GetOptionHandle { key, output } => {
                    let mut _args_array: [wl_argument; 3] = unsafe { ::std::mem::zeroed() };
                    let _arg_0 = ::std::ffi::CString::new(key).unwrap();
                    _args_array[0].s = _arg_0.as_ptr();
                    _args_array[1].o = output
                        .map(|o| o.as_ref().c_ptr() as *mut _)
                        .unwrap_or(::std::ptr::null_mut());
                    _args_array[2].o = ::std::ptr::null_mut() as *mut _;
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
    pub struct ZriverOptionsManagerV1(Proxy<ZriverOptionsManagerV1>);
    impl AsRef<Proxy<ZriverOptionsManagerV1>> for ZriverOptionsManagerV1 {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<ZriverOptionsManagerV1>> for ZriverOptionsManagerV1 {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            ZriverOptionsManagerV1(value)
        }
    }
    impl From<ZriverOptionsManagerV1> for Proxy<ZriverOptionsManagerV1> {
        #[inline]
        fn from(value: ZriverOptionsManagerV1) -> Self {
            value.0
        }
    }
    impl std::fmt::Debug for ZriverOptionsManagerV1 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(format_args!("{:?}", self.0))
        }
    }
    impl Interface for ZriverOptionsManagerV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zriver_options_manager_v1";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &zriver_options_manager_v1_interface }
        }
    }
    impl ZriverOptionsManagerV1 {
        #[doc = "destroy the zriver_options_manager_v1 object\n\nThis request indicates that the client will not use the manager object\nany more. Objects that have been created through this instance are\nnot affected.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        pub fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "get an option handle for the given key\n\nIf the output argument is non-null, the option is local to the given\noutput. Otherwise it is considered global."]
        pub fn get_option_handle(
            &self,
            key: String,
            output: Option<&super::wl_output::WlOutput>,
        ) -> Main<super::zriver_option_handle_v1::ZriverOptionHandleV1> {
            let msg = Request::GetOptionHandle {
                key: key,
                output: output.map(|o| o.clone()),
            };
            self.0.send(msg, None).unwrap()
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_GET_OPTION_HANDLE_SINCE: u32 = 1u32;
    static mut zriver_options_manager_v1_requests_get_option_handle_types: [*const wl_interface;
        3] = [
        NULLPTR as *const wl_interface,
        unsafe { &super::wl_output::wl_output_interface as *const wl_interface },
        unsafe {
            &super::zriver_option_handle_v1::zriver_option_handle_v1_interface
                as *const wl_interface
        },
    ];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zriver_options_manager_v1_requests: [wl_message; 2] = [
        wl_message {
            name: b"destroy\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"get_option_handle\0" as *const u8 as *const c_char,
            signature: b"s?on\0" as *const u8 as *const c_char,
            types: unsafe {
                &zriver_options_manager_v1_requests_get_option_handle_types as *const _
            },
        },
    ];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut zriver_options_manager_v1_interface: wl_interface = wl_interface {
        name: b"zriver_options_manager_v1\0" as *const u8 as *const c_char,
        version: 1,
        request_count: 2,
        requests: unsafe { &zriver_options_manager_v1_requests as *const _ },
        event_count: 0,
        events: NULLPTR as *const wl_message,
    };
}
#[doc = "handle to an option\n\nOn binding this object, one of the events will immediately be sent by\nthe server to inform the client of the current state of the option. New\nevents will be sent as the state changes."]
pub mod zriver_option_handle_v1 {
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
        #[doc = "destroy the handle\n\nThis request indicates that the client will not use the\nzriver_option_handle_v1 any more and that it may be safely destroyed.\n\nThis is a destructor, once sent this object cannot be used any longer."]
        Destroy,
        #[doc = "set the value of the option\n\nIf the option is either unset or set to a value of type int, this\nrequest asks the compositor to set the value of the option as well\nas the type if previously unset. The compositor is not required to\nhonor this request.\n\nIf the option is already set and is not of type int, this request does nothing."]
        SetIntValue { value: i32 },
        #[doc = "set the value of the option\n\nIf the option is either unset or set to a value of type uint, this\nrequest asks the compositor to set the value of the option as well\nas the type if previously unset. The compositor is not required to\nhonor this request.\n\nIf the option is already set and is not of type uint, this request\ndoes nothing."]
        SetUintValue { value: u32 },
        #[doc = "set the value of the option\n\nIf the option is either unset or set to a value of type fixed, this\nrequest asks the compositor to set the value of the option as well\nas the type if previously unset. The compositor is not required to\nhonor this request.\n\nIf the option is already set and is not of type fixed, this request\ndoes nothing."]
        SetFixedValue { value: f64 },
        #[doc = "set the value of the option\n\nIf the option is either unset or set to a value of type string,\nthis request asks the compositor to set the value of the option as\nwell as the type if previously unset. The compositor is not required\nto honor this request.\n\nIf the option is already set and is not of type string, this request\ndoes nothing."]
        SetStringValue { value: Option<String> },
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
                name: "set_int_value",
                since: 1,
                signature: &[super::ArgumentType::Int],
                destructor: false,
            },
            super::MessageDesc {
                name: "set_uint_value",
                since: 1,
                signature: &[super::ArgumentType::Uint],
                destructor: false,
            },
            super::MessageDesc {
                name: "set_fixed_value",
                since: 1,
                signature: &[super::ArgumentType::Fixed],
                destructor: false,
            },
            super::MessageDesc {
                name: "set_string_value",
                since: 1,
                signature: &[super::ArgumentType::Str],
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
                Request::SetIntValue { .. } => 1,
                Request::SetUintValue { .. } => 2,
                Request::SetFixedValue { .. } => 3,
                Request::SetStringValue { .. } => 4,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::Destroy => 1,
                Request::SetIntValue { .. } => 1,
                Request::SetUintValue { .. } => 1,
                Request::SetFixedValue { .. } => 1,
                Request::SetStringValue { .. } => 1,
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
                Request::SetIntValue { value } => Message {
                    sender_id: sender_id,
                    opcode: 1,
                    args: smallvec![Argument::Int(value),],
                },
                Request::SetUintValue { value } => Message {
                    sender_id: sender_id,
                    opcode: 2,
                    args: smallvec![Argument::Uint(value),],
                },
                Request::SetFixedValue { value } => Message {
                    sender_id: sender_id,
                    opcode: 3,
                    args: smallvec![Argument::Fixed((value * 256.) as i32),],
                },
                Request::SetStringValue { value } => Message {
                    sender_id: sender_id,
                    opcode: 4,
                    args: smallvec![Argument::Str(Box::new(unsafe {
                        ::std::ffi::CString::from_vec_unchecked(
                            value.map(Into::into).unwrap_or_else(Vec::new),
                        )
                    })),],
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
                Request::SetIntValue { value } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].i = value;
                    f(1, &mut _args_array)
                }
                Request::SetUintValue { value } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].u = value;
                    f(2, &mut _args_array)
                }
                Request::SetFixedValue { value } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].f = (value * 256.) as i32;
                    f(3, &mut _args_array)
                }
                Request::SetStringValue { value } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    let _arg_0 = value.map(|s| ::std::ffi::CString::new(s).unwrap());
                    _args_array[0].s = _arg_0.map(|s| s.as_ptr()).unwrap_or(::std::ptr::null());
                    f(4, &mut _args_array)
                }
            }
        }
    }
    #[derive(Debug)]
    #[non_exhaustive]
    pub enum Event {
        #[doc = "the option is currently unset\n\nThe option with this key has never been set, so the first set_*_value\nrequest received from any client will determine its type.\n\nThis can only ever be sent as the first event after binding this\ninterface as options cannot be unset once set."]
        Unset,
        #[doc = "the current value of the int option\n\nThis indicates to the client that the option is of type int as well\nas the current value of the option. Once set the type of the option\ncan never change."]
        IntValue { value: i32 },
        #[doc = "the current value of the uint option\n\nThis indicates to the client that the option is of type uint as well\nas the current value of the option. Once set the type of the option\ncan never change."]
        UintValue { value: u32 },
        #[doc = "the current value of the fixed option\n\nThis indicates to the client that the option is of type fixed as\nwell as the current value of the option. Once set the type of the option\ncan never change."]
        FixedValue { value: f64 },
        #[doc = "the current value of the string option\n\nThis indicates to the client that the option is of type string as\nwell as the current value of the option. Once set the type of the option\ncan never change."]
        StringValue { value: Option<String> },
    }
    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "unset",
                since: 1,
                signature: &[],
                destructor: false,
            },
            super::MessageDesc {
                name: "int_value",
                since: 1,
                signature: &[super::ArgumentType::Int],
                destructor: false,
            },
            super::MessageDesc {
                name: "uint_value",
                since: 1,
                signature: &[super::ArgumentType::Uint],
                destructor: false,
            },
            super::MessageDesc {
                name: "fixed_value",
                since: 1,
                signature: &[super::ArgumentType::Fixed],
                destructor: false,
            },
            super::MessageDesc {
                name: "string_value",
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
                Event::Unset => 0,
                Event::IntValue { .. } => 1,
                Event::UintValue { .. } => 2,
                Event::FixedValue { .. } => 3,
                Event::StringValue { .. } => 4,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Event::Unset => 1,
                Event::IntValue { .. } => 1,
                Event::UintValue { .. } => 1,
                Event::FixedValue { .. } => 1,
                Event::StringValue { .. } => 1,
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
                0 => Ok(Event::Unset),
                1 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::IntValue {
                        value: {
                            if let Some(Argument::Int(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                2 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::UintValue {
                        value: {
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
                    Ok(Event::FixedValue {
                        value: {
                            if let Some(Argument::Fixed(val)) = args.next() {
                                (val as f64) / 256.
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                4 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::StringValue {
                        value: {
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
                0 => Ok(Event::Unset),
                1 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::IntValue { value: _args[0].i })
                }
                2 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::UintValue { value: _args[0].u })
                }
                3 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::FixedValue {
                        value: (_args[0].f as f64) / 256.,
                    })
                }
                4 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::StringValue {
                        value: if _args[0].s.is_null() {
                            None
                        } else {
                            Some(
                                ::std::ffi::CStr::from_ptr(_args[0].s)
                                    .to_string_lossy()
                                    .into_owned(),
                            )
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
    pub struct ZriverOptionHandleV1(Proxy<ZriverOptionHandleV1>);
    impl AsRef<Proxy<ZriverOptionHandleV1>> for ZriverOptionHandleV1 {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<ZriverOptionHandleV1>> for ZriverOptionHandleV1 {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            ZriverOptionHandleV1(value)
        }
    }
    impl From<ZriverOptionHandleV1> for Proxy<ZriverOptionHandleV1> {
        #[inline]
        fn from(value: ZriverOptionHandleV1) -> Self {
            value.0
        }
    }
    impl std::fmt::Debug for ZriverOptionHandleV1 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(format_args!("{:?}", self.0))
        }
    }
    impl Interface for ZriverOptionHandleV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zriver_option_handle_v1";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &zriver_option_handle_v1_interface }
        }
    }
    impl ZriverOptionHandleV1 {
        #[doc = "destroy the handle\n\nThis request indicates that the client will not use the\nzriver_option_handle_v1 any more and that it may be safely destroyed.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        pub fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "set the value of the option\n\nIf the option is either unset or set to a value of type int, this\nrequest asks the compositor to set the value of the option as well\nas the type if previously unset. The compositor is not required to\nhonor this request.\n\nIf the option is already set and is not of type int, this request does nothing."]
        pub fn set_int_value(&self, value: i32) -> () {
            let msg = Request::SetIntValue { value: value };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "set the value of the option\n\nIf the option is either unset or set to a value of type uint, this\nrequest asks the compositor to set the value of the option as well\nas the type if previously unset. The compositor is not required to\nhonor this request.\n\nIf the option is already set and is not of type uint, this request\ndoes nothing."]
        pub fn set_uint_value(&self, value: u32) -> () {
            let msg = Request::SetUintValue { value: value };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "set the value of the option\n\nIf the option is either unset or set to a value of type fixed, this\nrequest asks the compositor to set the value of the option as well\nas the type if previously unset. The compositor is not required to\nhonor this request.\n\nIf the option is already set and is not of type fixed, this request\ndoes nothing."]
        pub fn set_fixed_value(&self, value: f64) -> () {
            let msg = Request::SetFixedValue { value: value };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "set the value of the option\n\nIf the option is either unset or set to a value of type string,\nthis request asks the compositor to set the value of the option as\nwell as the type if previously unset. The compositor is not required\nto honor this request.\n\nIf the option is already set and is not of type string, this request\ndoes nothing."]
        pub fn set_string_value(&self, value: Option<String>) -> () {
            let msg = Request::SetStringValue { value: value };
            self.0.send::<AnonymousObject>(msg, None);
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_INT_VALUE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_UINT_VALUE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_FIXED_VALUE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_STRING_VALUE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_UNSET_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_INT_VALUE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_UINT_VALUE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_FIXED_VALUE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_STRING_VALUE_SINCE: u32 = 1u32;
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zriver_option_handle_v1_requests: [wl_message; 5] = [
        wl_message {
            name: b"destroy\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"set_int_value\0" as *const u8 as *const c_char,
            signature: b"i\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"set_uint_value\0" as *const u8 as *const c_char,
            signature: b"u\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"set_fixed_value\0" as *const u8 as *const c_char,
            signature: b"f\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"set_string_value\0" as *const u8 as *const c_char,
            signature: b"?s\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
    ];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zriver_option_handle_v1_events: [wl_message; 5] = [
        wl_message {
            name: b"unset\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"int_value\0" as *const u8 as *const c_char,
            signature: b"i\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"uint_value\0" as *const u8 as *const c_char,
            signature: b"u\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"fixed_value\0" as *const u8 as *const c_char,
            signature: b"f\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"string_value\0" as *const u8 as *const c_char,
            signature: b"?s\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
    ];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut zriver_option_handle_v1_interface: wl_interface = wl_interface {
        name: b"zriver_option_handle_v1\0" as *const u8 as *const c_char,
        version: 1,
        request_count: 5,
        requests: unsafe { &zriver_option_handle_v1_requests as *const _ },
        event_count: 5,
        events: unsafe { &zriver_option_handle_v1_events as *const _ },
    };
}
