use super::display::{Context, Rectangle, Tag};
use crate::wayland::{
    river_layout_unstable_v1::{zriver_layout_v1, zriver_layout_v1::ZriverLayoutV1},
    river_options_unstable_v1::zriver_option_handle_v1,
    river_status_unstable_v1::zriver_output_status_v1,
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
    pub default_layout: Tag,
    pub tags: [Option<Tag>; 32],
}

#[derive(Copy, Clone)]
union Value {
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
                default_layout: Tag::new(),
                tags: Default::default(),
            }
        };
    }
    pub fn update(&mut self, context: Context) {
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
        self.zlayout
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
        self.get_option("output_layout", &context);
        self.get_option("frames_layout", &context);
        self.get_option("layout_per_tag", &context);
    }
    fn get_option(&mut self, name: &'static str, context: &Context) {
        let option_handle = context
            .options_manager
            .as_ref()
            .expect("Compositor doesn't implement river_options_unstable_v1")
            .get_option_handle(name.to_owned(), Some(context.output.as_ref().unwrap()));
        option_handle.quick_assign(move |option_handle, event, mut context| {
            let mut option_value: Value = Value { uint: 0 };
            let mut string: String = String::new();
            match event {
                zriver_option_handle_v1::Event::StringValue { value } => {
                    string = value.unwrap();
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
                    "output_layout" => {
                        match string.as_ref() {
                            "" => {
                                string = String::from("f");
                                option_handle.set_string_value(Some(string.clone()));
                            }
                            _ => {}
                        }
                        context
                            .get::<Context>()
                            .unwrap()
                            .options
                            .default_layout
                            .output = Options::layout_output(string);
                    }
                    "frames_layout" => {
                        match string.as_ref() {
                            "" => {
                                string = String::from("f");
                                option_handle.set_string_value(Some(string.clone()));
                            }
                            _ => {}
                        }
                        context
                            .get::<Context>()
                            .unwrap()
                            .options
                            .default_layout
                            .frames = Options::layout_frames(string);
                    }
                    "layout_per_tag" => {
                        context
                            .get::<Context>()
                            .unwrap()
                            .options
                            .parse_layout_per_tag(string);
                    }
                    _ => {}
                }
            }
        });
    }
    pub fn total_views(&self, view_available: u32) -> u32 {
        if self.main_amount >= self.view_amount {
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
        }
    }
    pub fn main_amount(&self, total_views: u32) -> u32 {
        if self.main_index + self.main_amount <= self.view_amount
            && total_views > 1
            && self.main_amount > 0
        {
            if self.main_index + self.main_amount > self.view_amount {
                self.view_amount - self.main_index
            } else {
                self.main_amount
            }
        } else {
            0
        }
    }
    pub fn layout_output(layout_output: String) -> Layout {
        let orientation;

        match layout_output.chars().next() {
            Some(c) => orientation = Options::layout(c),
            None => orientation = Layout::Full,
        }
        orientation
    }
    pub fn layout_frames(string: String) -> Vec<Layout> {
        let mut layout = Vec::new();

        for c in string.chars() {
            layout.push(Options::layout(c));
        }

        layout
    }
    pub fn set_focused(&mut self, tag: Tag) {
        self.tags[self.tagmask as usize] = Some(tag);
    }
    pub fn focused(&self) -> Option<Tag> {
        self.tags[self.tagmask as usize].clone()
    }
    fn layout(c: char) -> Layout {
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
        self.tags = Default::default();
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
                    let layout_output = match rule.next() {
                        Some(layout) => String::from(layout),
                        None => {
                            println!("Invalid layout");
                            break;
                        }
                    };
                    let layout_frames = match rule.next() {
                        Some(layout) => String::from(layout),
                        None => {
                            println!("Invalid layout");
                            break;
                        }
                    };
                    self.tags[tag as usize - 1] = Some({
                        Tag {
                            output: Options::layout_output(layout_output),
                            frames: Options::layout_frames(layout_frames),
                        }
                    });
                }
                None => break,
            }
        }
    }
    pub fn push_dimensions(&self, rect: &Rectangle) {
        self.zlayout.clone().unwrap().push_view_dimensions(
            self.serial,
            rect.x as i32,
            rect.y as i32,
            rect.w,
            rect.h,
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
        println!("    layout_output : {:?}", self.default_layout.output);
        println!("    layout_frames : {:?}", self.default_layout.frames);
        println!("\n  ZriverOutputStatusV1");
        println!("    tagmask : {}\n", self.tagmask);
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
