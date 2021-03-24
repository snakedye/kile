use super::display::{Context, Frame, State};
use crate::wayland::{
    river_layout_unstable_v1::{zriver_layout_v1, zriver_layout_v1::ZriverLayoutV1},
    river_options_unstable_v1::zriver_option_handle_v1,
    river_status_unstable_v1::{zriver_output_status_v1, zriver_seat_status_v1},
};
use wayland_client::DispatchData;
use wayland_client::Main;

#[derive(Clone, Debug)]
pub struct Options {
    pub serial: u32,
    pub tagmask: u32,
    pub zlayout: Option<Main<ZriverLayoutV1>>,
    pub view_amount: u32,
    pub usable_width: u32,
    pub usable_height: u32,
    pub view_padding: u32,
    pub outer_padding: u32,
    pub main_index: u32,
    pub main_factor: f64,
    pub main_amount: u32,
    pub layout_frame: String,
    pub layout_window: String,
    pub layout_per_tag: [Option<TagRule>; 32],
}

#[derive(Clone, Debug)]
pub struct TagRule {
    pub tag: u32,
    pub layout_frame: String,
    pub layout_window: String,
}

#[derive(Copy, Clone)]
pub union Value {
    double: f64,
    uint: u32,
}

#[derive(Copy, Clone, Debug)]
pub enum Layout {
    Tab,
    Full,
    Vertical,
    Horizontal,
}

impl Options {
    pub fn new() -> Options {
        return {
            Options {
                serial: 0,
                tagmask: 0,
                zlayout: None,
                view_amount: 0,
                view_padding: 0,
                outer_padding: 0,
                usable_width: 0,
                usable_height: 0,
                main_factor: 0.0,
                main_index: 0,
                main_amount: 0,
                layout_window: String::new(),
                layout_frame: String::new(),
                layout_per_tag: Default::default(),
            }
        };
    }
    pub fn init(&mut self, context: Context) {

        let output_status = context
            .status_manager
            .as_ref()
            .expect("Compositor doesn't implement river_status_unstable_v1")
            .get_river_output_status(&context.output.clone().unwrap());

        output_status.quick_assign(move |_, event, mut context| match event {
            zriver_output_status_v1::Event::FocusedTags { tags } => {
                context.get::<Context>().unwrap().options.tagmask = tag_index(tags);
                match &context.get::<Context>().unwrap().options.zlayout {
                    Some(zlayout) => zlayout.parameters_changed(),
                    None => {}
                }
            }
            zriver_output_status_v1::Event::ViewTags { tags } => {}
        });

        self.zlayout = Some(
            context
                .clone()
                .layout_manager
                .expect("Compositor doesn't implement river_layout_unstable_v1")
                .get_river_layout(context.output.as_ref().unwrap(), context.namespace.clone()),
        );
        self .zlayout
            .as_ref()
            .unwrap()
            .quick_assign(move |_, event, mut context: DispatchData| match event {
                zriver_layout_v1::Event::LayoutDemand {
                    view_amount,
                    usable_width,
                    usable_height,
                    serial,
                } => {
                    context.get::<Context>().unwrap().options.serial = serial;
                    context.get::<Context>().unwrap().options.view_amount = view_amount;
                    context.get::<Context>().unwrap().options.usable_height = usable_height;
                    context.get::<Context>().unwrap().options.usable_width = usable_width;
                }
                zriver_layout_v1::Event::AdvertiseView {
                    tags,
                    app_id,
                    serial,
                } => {}
                zriver_layout_v1::Event::NamespaceInUse => {
                    println!("Namespace already in use.");
                    context.get::<Context>().unwrap().running = false;
                }
                zriver_layout_v1::Event::AdvertiseDone { serial } => {}
            });

        self.get_option("main_factor", &context);
        self.get_option("main_amount", &context);
        self.get_option("main_index", &context);
        self.get_option("view_padding", &context);
        self.get_option("outer_padding", &context);
        self.get_option("kile_frame", &context);
        self.get_option("kile_window", &context);
        self.get_option("layout_per_tag", &context);
    }
    fn get_option(&mut self, name: &'static str, context: &Context) {
        let option = context
            .options_manager
            .as_ref()
            .expect("Compositor doesn't implement river_options_unstable_v1")
            .get_option_handle(name.to_owned(), Some(context.output.as_ref().unwrap()));
        option.quick_assign(move |_, event, mut context| {
            let mut option_value: Value = Value { uint: 0 };
            let mut args: String = String::new();
            match event {
                zriver_option_handle_v1::Event::StringValue { value } => {
                    args = value.unwrap();
                }
                zriver_option_handle_v1::Event::FixedValue { value } => option_value.double = value,
                zriver_option_handle_v1::Event::UintValue { value } => option_value.uint = value,
                zriver_option_handle_v1::Event::IntValue { value } => {
                    if value < 0 {
                        option_value.uint = 0;
                    } else {
                        option_value.uint = value as u32;
                    }
                }
                zriver_option_handle_v1::Event::Unset => {}
            }
            match &context.get::<Context>().unwrap().options.zlayout {
                Some(zlayout) => zlayout.parameters_changed(),
                None => {}
            }
            unsafe {
                match name {
                    "main_index" => {
                        context.get::<Context>().unwrap().options.main_index = option_value.uint
                    }
                    "main_amount" => {
                        context.get::<Context>().unwrap().options.main_amount = option_value.uint
                    }
                    "main_factor" => {
                        context.get::<Context>().unwrap().options.main_factor = option_value.double
                    }
                    "view_padding" => {
                        context.get::<Context>().unwrap().options.view_padding = option_value.uint
                    }
                    "outer_padding" => {
                        context.get::<Context>().unwrap().options.outer_padding = option_value.uint
                    }
                    "kile_frame" => {
                        context.get::<Context>().unwrap().options.layout_frame = args;
                    }
                    "kile_window" => {
                        context.get::<Context>().unwrap().options.layout_window = args;
                    }
                    "layout_per_tag" => {
                        context
                            .get::<Context>()
                            .unwrap()
                            .options
                            .parse_layout_per_tag(args);
                    }
                    _ => {}
                }
            }
        });
    }
    pub fn layout_frame(&self, layout_frame: String, view_available: u32) -> (Layout, u32, State) {
        let mut orientation = Layout::Full;
        let total_views = if self.main_amount >= self.view_amount {
            1
        } else if self.view_amount >= view_available {
            if 1 + self.view_amount - self.main_amount < view_available
                && self.main_amount > self.view_amount / view_available
            {
                1 + self.view_amount - self.main_amount
            } else {
                view_available
            }
        } else {
            self.view_amount
        };
        for c in layout_frame.chars() {
            orientation = Options::layout_parse(&c);
        }
        (orientation, total_views, State::Frame)
    }
    pub fn layout_window(&self, string: String, total_views: u32) -> Vec<(Layout, u32, State)> {
        let mut layout = Vec::new();
        let main_view = if self.main_index + self.main_amount <= self.view_amount
            && self.main_index < total_views
            && total_views > 1
            && self.main_amount > 0
        {
            1
        } else {
            0
        };
        let main_amount = if self.main_index < total_views
            && self.main_index + self.main_amount > self.view_amount
        {
            self.view_amount - self.main_index
        } else {
            self.main_amount
        };
        let mut reste = (self.view_amount - main_view * main_amount) % (total_views - main_view);
        let client_count = if total_views > 1 {
            (self.view_amount - main_view * main_amount) / (total_views - main_view)
        } else {
            self.view_amount
        };

        for (i, c) in string.chars().enumerate() {
            let orientation = Options::layout_parse(&c);
            if i == self.main_index as usize && total_views > 1 && main_view == 1 {
                layout.push((orientation, main_amount, State::Window));
            } else {
                layout.push((
                    orientation,
                    if reste > 0 {
                        reste -= 1;
                        client_count + 1
                    } else {
                        client_count
                    },
                    State::Window,
                ));
            }
            if i > total_views as usize {
                break;
            }
        }

        return layout;
    }
    fn layout_parse(c: &char) -> Layout {
        match c {
            'v' => Layout::Vertical,
            'h' => Layout::Horizontal,
            't' => Layout::Tab,
            'f' => Layout::Full,
            _ => {
                println!("{}: Not a valid character at index", c);
                Layout::Full
            }
        }
    }
    pub fn parse_layout_per_tag(&mut self, layout_per_tag: String) {
        let mut layout_per_tag = layout_per_tag.split_whitespace();
        self.layout_per_tag = Default::default();
        loop {
            match layout_per_tag.next() {
                Some(rule) => {
                    let mut rule = rule.split(':');
                    let tag: u32 = match rule.next() {
                        Some(tag) => match tag.parse::<u32>() {
                            Ok(int) => {
                                if int > 0 {
                                    int
                                } else {
                                    println!("Invalid tag");
                                    break;
                                }
                            }
                            Err(_t) => {
                                println!("Invalid tag");
                                break;
                            }
                        },
                        None => {
                            println!("Invalid format");
                            break;
                        }
                    };
                    let layout_frame = match rule.next() {
                        Some(layout) => String::from(layout),
                        None => {
                            println!("Invalid layout");
                            break;
                        }
                    };
                    let layout_window = match rule.next() {
                        Some(layout) => String::from(layout),
                        None => {
                            println!("Invalid layout");
                            break;
                        }
                    };
                    self.layout_per_tag[tag as usize - 1] = Some({
                        TagRule {
                            tag: tag,
                            layout_frame: layout_frame,
                            layout_window: layout_window,
                        }
                    });
                }
                None => break,
            }
        }
    }
    pub fn push_dimensions(&self, frame: &Frame) {
        self.zlayout.clone().unwrap().push_view_dimensions(
            self.serial,
            frame.x as i32,
            frame.y as i32,
            frame.w,
            frame.h,
        )
    }
    pub fn debug(&self) {
        println!("Option - {}", self.serial);
        println!("\n  ZriverLayoutV1");
        println!("    view_amount : {}", self.view_amount);
        println!("    usable_width : {}", self.usable_width);
        println!("    usable_height : {}", self.usable_height);
        println!("\n  ZriverOptionHandleV1");
        println!("    outer_padding : {}", self.outer_padding);
        println!("    view_padding : {}", self.view_padding);
        println!("    main_factor : {}", self.main_factor);
        println!("    main_index : {}", self.main_index);
        println!("    main_amount : {}", self.main_amount);
        println!("    kile_window : {}", self.layout_window);
        println!("    kile_frame : {}", self.layout_frame);
        println!("\n  ZriverOutputStatusV1");
        println!("    tagmask : {}", self.tagmask);
    }
    pub fn commit(&self) {
        self.zlayout.clone().unwrap().commit(self.serial);
    }
}

pub fn tag_index(mut tagmask: u32) -> u32 {
    let mut tag = 0;
    while tagmask / 2 >= 1 {
        tagmask /= 2;
        tag += 1;
    }
    tag
}

