use super::display::{Context, Frame};
use crate::wayland::{
    river_layout_unstable_v1::{
        zriver_layout_manager_v1::ZriverLayoutManagerV1, zriver_layout_v1,
        zriver_layout_v1::ZriverLayoutV1,
    },
    river_options_unstable_v1::{
        zriver_option_handle_v1, zriver_option_handle_v1::ZriverOptionHandleV1,
        zriver_options_manager_v1::ZriverOptionsManagerV1,
    },
};
use wayland_client::protocol::wl_output::WlOutput;
use wayland_client::Main;

pub struct Options {
    pub serial: u32,
    pub tagmask: u32,
    pub view_amount: u32,
    pub usable_width: u32,
    pub usable_height: u32,
    pub view_padding: u32,
    pub output_padding: u32,
    pub main_index: u32,
    pub main_factor: f64,
    pub main_count: u32,
    pub capacity: u32,
    pub arguments: Vec<Layout>,
}

#[derive(Copy, Clone)]
pub union Value {
    double: f64,
    uint: u32,
    int: i32,
}

#[derive(Copy, Clone, Debug)]
pub enum Layout {
    Tab,
    Full,
    Vertical,
    Horizontal,
}

impl Options {
    // Listen to the options and layout and returns an Options when the context is updated
    pub fn new(context: Context) -> Options {
        let mut serial_value: u32 = 0;
        let mut view_amount_value: u32 = 0;
        let mut usable_width_value: u32 = 0;
        let mut usable_height_value: u32 = 0;
        let mut namespace_status = true;

        let layout = context
            .clone()
            .layout_manager
            .unwrap()
            .get_river_layout(&context.output, context.namespace.clone());
        layout.quick_assign(move |_, event, _| match event {
            zriver_layout_v1::Event::LayoutDemand {
                view_amount,
                usable_width,
                usable_height,
                serial,
            } => {
                serial_value = serial;
                view_amount_value = view_amount;
                usable_height_value = usable_height;
                usable_width_value = usable_width;
            }
            zriver_layout_v1::Event::AdvertiseView {
                tags,
                app_id,
                serial,
            } => {}
            zriver_layout_v1::Event::NamespaceInUse => {
                namespace_status = false;
                // println!("Namespace already in use.");
                // layout.destroy()
            }
            zriver_layout_v1::Event::AdvertiseDone { serial } => {}
        });

        if !namespace_status {
            layout.destroy()
        }

        let mut options = {
            Options {
                serial: serial_value,
                tagmask: 0,
                view_amount: view_amount_value,
                capacity: 0,
                view_padding: 0,
                output_padding: 0,
                usable_width: usable_width_value,
                usable_height: usable_height_value,
                main_factor: 0.0,
                main_index: 0,
                main_count: 0,
                arguments: Vec::new(),
            }
        };

        options.get_option("main-factor", &context);
        options.get_option("main-count", &context);
        options.get_option("main-index", &context);
        options.get_option("view-padding", &context);
        options.get_option("capacity", &context);
        options.get_option("layout", &context);
        // options.get_option("layout", context);
        // "ver hor hor"

        return options;
    }
    fn get_option(&mut self, name: &str, context: &Context) {
        let mut layout_string: String = String::new();
        let option = context
            .options_manager
            .clone()
            .unwrap()
            .get_option_handle(String::from(name), Some(&context.output));
        unsafe {
            let mut option_value: Value = Value { uint: 0 };
            option.quick_assign(move |option, event, _| match event {
                zriver_option_handle_v1::Event::StringValue { value } => {
                    layout_string.push_str(&value.unwrap())
                }
                zriver_option_handle_v1::Event::FixedValue { value } => option_value.double = value,
                zriver_option_handle_v1::Event::UintValue { value } => option_value.uint = value,
                zriver_option_handle_v1::Event::IntValue { value } => {
                    if value < 0 {
                        option_value.int = 0;
                    } else {
                        option_value.int = value;
                    }
                }
                zriver_option_handle_v1::Event::Unset => {}
            });
            match name {
                "main-index" => (*self).main_index = option_value.uint,
                "main-count" => (*self).main_count = option_value.uint,
                "main-factor" => (*self).main_factor = option_value.double,
                "view-padding" => (*self).view_padding = option_value.uint,
                "output-padding" => (*self).output_padding = option_value.uint,
                "capacity" => (*self).capacity = option_value.uint,
                // "layout"=>(*self).arguments=parse_layout(layout_string),
                _ => {}
            }
        }
    }
}

fn parse_layout(value: String) -> Vec<Layout> {
    let mut iter = value.split_whitespace();
    let mut vec: Vec<Layout> = Vec::new();
    fn to_layout(str: &str) -> Layout {
        match str {
            "ver" => Layout::Vertical,
            "hor" => Layout::Horizontal,
            "tab" => Layout::Tab,
            "ful" => Layout::Full,
            _ => Layout::Full,
        }
    }
    while iter.next() != None {
        vec.push(iter.next().map(to_layout).unwrap());
    }

    return vec;
}
