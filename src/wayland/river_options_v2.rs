use std::os::raw::{c_char, c_void};
const NULLPTR: *const c_void = 0 as *const c_void;
static mut types_null: [*const sys::common::wl_interface; 2] = [
    NULLPTR as *const sys::common::wl_interface,
    NULLPTR as *const sys::common::wl_interface,
];
#[doc = "declare options and get handles\n\nThis interface allows clients to declare new options and create\nriver_option_v2 handle objects in order to retrieve the current\nvalue or set a new one."]
pub mod river_options_manager_v2 {
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
        #[doc = "destroy the river_options_manager_v2 object\n\nThis request indicates that the client will not use the manager object\nany more. Objects that have been created through this instance are\nnot affected.\n\nThis is a destructor, once sent this object cannot be used any longer."]
        Destroy,
        #[doc = "declare a new option\n\nThe option is created in the global scope and is initialized with the\nprovided value. This request is ignored if the option already exists."]
        DeclareIntOption { key: String, value: i32 },
        #[doc = "declare a new option\n\nThe option is created in the global scope and is initialized with the\nprovided value. This request is ignored if the option already exists."]
        DeclareUintOption { key: String, value: u32 },
        #[doc = "declare a new option\n\nThe option is created in the global scope and is initialized with the\nprovided value. This request is ignored if the option already exists."]
        DeclareStringOption { key: String, value: Option<String> },
        #[doc = "declare a new option\n\nThe option is created in the global scope and is initialized with the\nprovided value. This request is ignored if the option already exists."]
        DeclareFixedOption { key: String, value: f64 },
        #[doc = "get an option handle for the given key\n\nIf the output argument is non-null, the option is local to the given\noutput. Otherwise it is considered global."]
        GetOptionHandle {
            key: String,
            output: Option<super::wl_output::WlOutput>,
        },
        #[doc = "unset an output-local value if any\n\nThis causes the value of the option for the given output to fall\nback to the global value."]
        UnsetOption {
            key: String,
            output: super::wl_output::WlOutput,
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
                name: "declare_int_option",
                since: 1,
                signature: &[super::ArgumentType::Str, super::ArgumentType::Int],
                destructor: false,
            },
            super::MessageDesc {
                name: "declare_uint_option",
                since: 1,
                signature: &[super::ArgumentType::Str, super::ArgumentType::Uint],
                destructor: false,
            },
            super::MessageDesc {
                name: "declare_string_option",
                since: 1,
                signature: &[super::ArgumentType::Str, super::ArgumentType::Str],
                destructor: false,
            },
            super::MessageDesc {
                name: "declare_fixed_option",
                since: 1,
                signature: &[super::ArgumentType::Str, super::ArgumentType::Fixed],
                destructor: false,
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
            super::MessageDesc {
                name: "unset_option",
                since: 1,
                signature: &[super::ArgumentType::Str, super::ArgumentType::Object],
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
                Request::DeclareIntOption { .. } => 1,
                Request::DeclareUintOption { .. } => 2,
                Request::DeclareStringOption { .. } => 3,
                Request::DeclareFixedOption { .. } => 4,
                Request::GetOptionHandle { .. } => 5,
                Request::UnsetOption { .. } => 6,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::Destroy => 1,
                Request::DeclareIntOption { .. } => 1,
                Request::DeclareUintOption { .. } => 1,
                Request::DeclareStringOption { .. } => 1,
                Request::DeclareFixedOption { .. } => 1,
                Request::GetOptionHandle { .. } => 1,
                Request::UnsetOption { .. } => 1,
            }
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                5 => Some(Object::from_interface::<
                    super::river_option_handle_v2::RiverOptionHandleV2,
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
                Request::DeclareIntOption { key, value } => Message {
                    sender_id: sender_id,
                    opcode: 1,
                    args: smallvec![
                        Argument::Str(Box::new(unsafe {
                            ::std::ffi::CString::from_vec_unchecked(key.into())
                        })),
                        Argument::Int(value),
                    ],
                },
                Request::DeclareUintOption { key, value } => Message {
                    sender_id: sender_id,
                    opcode: 2,
                    args: smallvec![
                        Argument::Str(Box::new(unsafe {
                            ::std::ffi::CString::from_vec_unchecked(key.into())
                        })),
                        Argument::Uint(value),
                    ],
                },
                Request::DeclareStringOption { key, value } => Message {
                    sender_id: sender_id,
                    opcode: 3,
                    args: smallvec![
                        Argument::Str(Box::new(unsafe {
                            ::std::ffi::CString::from_vec_unchecked(key.into())
                        })),
                        Argument::Str(Box::new(unsafe {
                            ::std::ffi::CString::from_vec_unchecked(
                                value.map(Into::into).unwrap_or_else(Vec::new),
                            )
                        })),
                    ],
                },
                Request::DeclareFixedOption { key, value } => Message {
                    sender_id: sender_id,
                    opcode: 4,
                    args: smallvec![
                        Argument::Str(Box::new(unsafe {
                            ::std::ffi::CString::from_vec_unchecked(key.into())
                        })),
                        Argument::Fixed((value * 256.) as i32),
                    ],
                },
                Request::GetOptionHandle { key, output } => Message {
                    sender_id: sender_id,
                    opcode: 5,
                    args: smallvec![
                        Argument::Str(Box::new(unsafe {
                            ::std::ffi::CString::from_vec_unchecked(key.into())
                        })),
                        Argument::Object(output.map(|o| o.as_ref().id()).unwrap_or(0)),
                        Argument::NewId(0),
                    ],
                },
                Request::UnsetOption { key, output } => Message {
                    sender_id: sender_id,
                    opcode: 6,
                    args: smallvec![
                        Argument::Str(Box::new(unsafe {
                            ::std::ffi::CString::from_vec_unchecked(key.into())
                        })),
                        Argument::Object(output.as_ref().id()),
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
                Request::DeclareIntOption { key, value } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    let _arg_0 = ::std::ffi::CString::new(key).unwrap();
                    _args_array[0].s = _arg_0.as_ptr();
                    _args_array[1].i = value;
                    f(1, &mut _args_array)
                }
                Request::DeclareUintOption { key, value } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    let _arg_0 = ::std::ffi::CString::new(key).unwrap();
                    _args_array[0].s = _arg_0.as_ptr();
                    _args_array[1].u = value;
                    f(2, &mut _args_array)
                }
                Request::DeclareStringOption { key, value } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    let _arg_0 = ::std::ffi::CString::new(key).unwrap();
                    _args_array[0].s = _arg_0.as_ptr();
                    let _arg_1 = value.map(|s| ::std::ffi::CString::new(s).unwrap());
                    _args_array[1].s = _arg_1.map(|s| s.as_ptr()).unwrap_or(::std::ptr::null());
                    f(3, &mut _args_array)
                }
                Request::DeclareFixedOption { key, value } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    let _arg_0 = ::std::ffi::CString::new(key).unwrap();
                    _args_array[0].s = _arg_0.as_ptr();
                    _args_array[1].f = (value * 256.) as i32;
                    f(4, &mut _args_array)
                }
                Request::GetOptionHandle { key, output } => {
                    let mut _args_array: [wl_argument; 3] = unsafe { ::std::mem::zeroed() };
                    let _arg_0 = ::std::ffi::CString::new(key).unwrap();
                    _args_array[0].s = _arg_0.as_ptr();
                    _args_array[1].o = output
                        .map(|o| o.as_ref().c_ptr() as *mut _)
                        .unwrap_or(::std::ptr::null_mut());
                    _args_array[2].o = ::std::ptr::null_mut() as *mut _;
                    f(5, &mut _args_array)
                }
                Request::UnsetOption { key, output } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    let _arg_0 = ::std::ffi::CString::new(key).unwrap();
                    _args_array[0].s = _arg_0.as_ptr();
                    _args_array[1].o = output.as_ref().c_ptr() as *mut _;
                    f(6, &mut _args_array)
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
    pub struct RiverOptionsManagerV2(Proxy<RiverOptionsManagerV2>);
    impl AsRef<Proxy<RiverOptionsManagerV2>> for RiverOptionsManagerV2 {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<RiverOptionsManagerV2>> for RiverOptionsManagerV2 {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            RiverOptionsManagerV2(value)
        }
    }
    impl From<RiverOptionsManagerV2> for Proxy<RiverOptionsManagerV2> {
        #[inline]
        fn from(value: RiverOptionsManagerV2) -> Self {
            value.0
        }
    }
    impl std::fmt::Debug for RiverOptionsManagerV2 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(format_args!("{:?}", self.0))
        }
    }
    impl Interface for RiverOptionsManagerV2 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "river_options_manager_v2";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &river_options_manager_v2_interface }
        }
    }
    impl RiverOptionsManagerV2 {
        #[doc = "destroy the river_options_manager_v2 object\n\nThis request indicates that the client will not use the manager object\nany more. Objects that have been created through this instance are\nnot affected.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        pub fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "declare a new option\n\nThe option is created in the global scope and is initialized with the\nprovided value. This request is ignored if the option already exists."]
        pub fn declare_int_option(&self, key: String, value: i32) -> () {
            let msg = Request::DeclareIntOption {
                key: key,
                value: value,
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "declare a new option\n\nThe option is created in the global scope and is initialized with the\nprovided value. This request is ignored if the option already exists."]
        pub fn declare_uint_option(&self, key: String, value: u32) -> () {
            let msg = Request::DeclareUintOption {
                key: key,
                value: value,
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "declare a new option\n\nThe option is created in the global scope and is initialized with the\nprovided value. This request is ignored if the option already exists."]
        pub fn declare_string_option(&self, key: String, value: Option<String>) -> () {
            let msg = Request::DeclareStringOption {
                key: key,
                value: value,
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "declare a new option\n\nThe option is created in the global scope and is initialized with the\nprovided value. This request is ignored if the option already exists."]
        pub fn declare_fixed_option(&self, key: String, value: f64) -> () {
            let msg = Request::DeclareFixedOption {
                key: key,
                value: value,
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "get an option handle for the given key\n\nIf the output argument is non-null, the option is local to the given\noutput. Otherwise it is considered global."]
        pub fn get_option_handle(
            &self,
            key: String,
            output: Option<&super::wl_output::WlOutput>,
        ) -> Main<super::river_option_handle_v2::RiverOptionHandleV2> {
            let msg = Request::GetOptionHandle {
                key: key,
                output: output.map(|o| o.clone()),
            };
            self.0.send(msg, None).unwrap()
        }
        #[doc = "unset an output-local value if any\n\nThis causes the value of the option for the given output to fall\nback to the global value."]
        pub fn unset_option(&self, key: String, output: &super::wl_output::WlOutput) -> () {
            let msg = Request::UnsetOption {
                key: key,
                output: output.clone(),
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DECLARE_INT_OPTION_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DECLARE_UINT_OPTION_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DECLARE_STRING_OPTION_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DECLARE_FIXED_OPTION_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_GET_OPTION_HANDLE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_UNSET_OPTION_SINCE: u32 = 1u32;
    static mut river_options_manager_v2_requests_get_option_handle_types: [*const wl_interface; 3] = [
        NULLPTR as *const wl_interface,
        unsafe { &super::wl_output::wl_output_interface as *const wl_interface },
        unsafe {
            &super::river_option_handle_v2::river_option_handle_v2_interface as *const wl_interface
        },
    ];
    static mut river_options_manager_v2_requests_unset_option_types: [*const wl_interface; 2] =
        [NULLPTR as *const wl_interface, unsafe {
            &super::wl_output::wl_output_interface as *const wl_interface
        }];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut river_options_manager_v2_requests: [wl_message; 7] = [
        wl_message {
            name: b"destroy\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"declare_int_option\0" as *const u8 as *const c_char,
            signature: b"si\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"declare_uint_option\0" as *const u8 as *const c_char,
            signature: b"su\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"declare_string_option\0" as *const u8 as *const c_char,
            signature: b"s?s\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"declare_fixed_option\0" as *const u8 as *const c_char,
            signature: b"sf\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"get_option_handle\0" as *const u8 as *const c_char,
            signature: b"s?on\0" as *const u8 as *const c_char,
            types: unsafe {
                &river_options_manager_v2_requests_get_option_handle_types as *const _
            },
        },
        wl_message {
            name: b"unset_option\0" as *const u8 as *const c_char,
            signature: b"so\0" as *const u8 as *const c_char,
            types: unsafe { &river_options_manager_v2_requests_unset_option_types as *const _ },
        },
    ];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut river_options_manager_v2_interface: wl_interface = wl_interface {
        name: b"river_options_manager_v2\0" as *const u8 as *const c_char,
        version: 1,
        request_count: 7,
        requests: unsafe { &river_options_manager_v2_requests as *const _ },
        event_count: 0,
        events: NULLPTR as *const wl_message,
    };
}
#[doc = "handle to an option\n\nOn binding this object, one of the events will immediately be sent by\nthe server to inform the client of the current state of the option,\nincluding its type. Making one of the 4 set requests before receiving\nthis first event would be a bug as the client would not yet know the\ntype of the option.  New events will be sent as the state changes."]
pub mod river_option_handle_v2 {
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
        #[doc = "a request other than destroy was made after receiving the undeclared event"]
        RequestWhileUndeclared = 0,
        #[doc = "a set request of the wrong type was made"]
        TypeMismatch = 1,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                0 => Some(Error::RequestWhileUndeclared),
                1 => Some(Error::TypeMismatch),
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
        #[doc = "destroy the handle\n\nThis request indicates that the client will not use the\nriver_option_handle_v2 any more and that it may be safely destroyed.\n\nThis is a destructor, once sent this object cannot be used any longer."]
        Destroy,
        #[doc = "set the value of the option\n\nIf the option is of type int, set the value of the option.\nOtherwise, this request is a protocol error."]
        SetIntValue { value: i32 },
        #[doc = "set the value of the option\n\nIf the option is of type uint, set the value of the option.\nOtherwise, this request is a protocol error."]
        SetUintValue { value: u32 },
        #[doc = "set the value of the option\n\nIf the option is of type string, set the value of the option.\nOtherwise, this request is a protocol error."]
        SetStringValue { value: Option<String> },
        #[doc = "set the value of the option\n\nIf the option is of type fixed, set the value of the option.\nOtherwise, this request is a protocol error."]
        SetFixedValue { value: f64 },
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
                name: "set_string_value",
                since: 1,
                signature: &[super::ArgumentType::Str],
                destructor: false,
            },
            super::MessageDesc {
                name: "set_fixed_value",
                since: 1,
                signature: &[super::ArgumentType::Fixed],
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
                Request::SetStringValue { .. } => 3,
                Request::SetFixedValue { .. } => 4,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::Destroy => 1,
                Request::SetIntValue { .. } => 1,
                Request::SetUintValue { .. } => 1,
                Request::SetStringValue { .. } => 1,
                Request::SetFixedValue { .. } => 1,
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
                Request::SetStringValue { value } => Message {
                    sender_id: sender_id,
                    opcode: 3,
                    args: smallvec![Argument::Str(Box::new(unsafe {
                        ::std::ffi::CString::from_vec_unchecked(
                            value.map(Into::into).unwrap_or_else(Vec::new),
                        )
                    })),],
                },
                Request::SetFixedValue { value } => Message {
                    sender_id: sender_id,
                    opcode: 4,
                    args: smallvec![Argument::Fixed((value * 256.) as i32),],
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
                Request::SetStringValue { value } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    let _arg_0 = value.map(|s| ::std::ffi::CString::new(s).unwrap());
                    _args_array[0].s = _arg_0.map(|s| s.as_ptr()).unwrap_or(::std::ptr::null());
                    f(3, &mut _args_array)
                }
                Request::SetFixedValue { value } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].f = (value * 256.) as i32;
                    f(4, &mut _args_array)
                }
            }
        }
    }
    #[derive(Debug)]
    #[non_exhaustive]
    pub enum Event {
        #[doc = "the option has never been declared\n\nNo option with the the given name has ever been declared. All requests\non this object aside from the destroy request are a protocol error and\nno further events will be sent."]
        Undeclared,
        #[doc = "the current value of the int option\n\nThis indicates to the client that the option is of type int as well\nas the current value of the option. Once set the type of the option\ncan never change."]
        IntValue { value: i32 },
        #[doc = "the current value of the uint option\n\nThis indicates to the client that the option is of type uint as well\nas the current value of the option. Once set the type of the option\ncan never change."]
        UintValue { value: u32 },
        #[doc = "the current value of the string option\n\nThis indicates to the client that the option is of type string as well\nas the current value of the option. Once set the type of the option\ncan never change."]
        StringValue { value: Option<String> },
        #[doc = "the current value of the fixed option\n\nThis indicates to the client that the option is of type fixed as well\nas the current value of the option. Once set the type of the option\ncan never change."]
        FixedValue { value: f64 },
    }
    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "undeclared",
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
                name: "string_value",
                since: 1,
                signature: &[super::ArgumentType::Str],
                destructor: false,
            },
            super::MessageDesc {
                name: "fixed_value",
                since: 1,
                signature: &[super::ArgumentType::Fixed],
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
                Event::Undeclared => 0,
                Event::IntValue { .. } => 1,
                Event::UintValue { .. } => 2,
                Event::StringValue { .. } => 3,
                Event::FixedValue { .. } => 4,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Event::Undeclared => 1,
                Event::IntValue { .. } => 1,
                Event::UintValue { .. } => 1,
                Event::StringValue { .. } => 1,
                Event::FixedValue { .. } => 1,
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
                0 => Ok(Event::Undeclared),
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
                4 => {
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
                0 => Ok(Event::Undeclared),
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
                4 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::FixedValue {
                        value: (_args[0].f as f64) / 256.,
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
    pub struct RiverOptionHandleV2(Proxy<RiverOptionHandleV2>);
    impl AsRef<Proxy<RiverOptionHandleV2>> for RiverOptionHandleV2 {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<RiverOptionHandleV2>> for RiverOptionHandleV2 {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            RiverOptionHandleV2(value)
        }
    }
    impl From<RiverOptionHandleV2> for Proxy<RiverOptionHandleV2> {
        #[inline]
        fn from(value: RiverOptionHandleV2) -> Self {
            value.0
        }
    }
    impl std::fmt::Debug for RiverOptionHandleV2 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(format_args!("{:?}", self.0))
        }
    }
    impl Interface for RiverOptionHandleV2 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "river_option_handle_v2";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &river_option_handle_v2_interface }
        }
    }
    impl RiverOptionHandleV2 {
        #[doc = "destroy the handle\n\nThis request indicates that the client will not use the\nriver_option_handle_v2 any more and that it may be safely destroyed.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        pub fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "set the value of the option\n\nIf the option is of type int, set the value of the option.\nOtherwise, this request is a protocol error."]
        pub fn set_int_value(&self, value: i32) -> () {
            let msg = Request::SetIntValue { value: value };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "set the value of the option\n\nIf the option is of type uint, set the value of the option.\nOtherwise, this request is a protocol error."]
        pub fn set_uint_value(&self, value: u32) -> () {
            let msg = Request::SetUintValue { value: value };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "set the value of the option\n\nIf the option is of type string, set the value of the option.\nOtherwise, this request is a protocol error."]
        pub fn set_string_value(&self, value: Option<String>) -> () {
            let msg = Request::SetStringValue { value: value };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "set the value of the option\n\nIf the option is of type fixed, set the value of the option.\nOtherwise, this request is a protocol error."]
        pub fn set_fixed_value(&self, value: f64) -> () {
            let msg = Request::SetFixedValue { value: value };
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
    pub const REQ_SET_STRING_VALUE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_FIXED_VALUE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_UNDECLARED_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_INT_VALUE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_UINT_VALUE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_STRING_VALUE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_FIXED_VALUE_SINCE: u32 = 1u32;
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut river_option_handle_v2_requests: [wl_message; 5] = [
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
            name: b"set_string_value\0" as *const u8 as *const c_char,
            signature: b"?s\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"set_fixed_value\0" as *const u8 as *const c_char,
            signature: b"f\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
    ];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut river_option_handle_v2_events: [wl_message; 5] = [
        wl_message {
            name: b"undeclared\0" as *const u8 as *const c_char,
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
            name: b"string_value\0" as *const u8 as *const c_char,
            signature: b"?s\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"fixed_value\0" as *const u8 as *const c_char,
            signature: b"f\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
    ];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut river_option_handle_v2_interface: wl_interface = wl_interface {
        name: b"river_option_handle_v2\0" as *const u8 as *const c_char,
        version: 1,
        request_count: 5,
        requests: unsafe { &river_option_handle_v2_requests as *const _ },
        event_count: 5,
        events: unsafe { &river_option_handle_v2_events as *const _ },
    };
}
