use super::options::{Layout, Options};
use crate::wayland::{
    river_layout_unstable_v1::zriver_layout_manager_v1::ZriverLayoutManagerV1,
    river_options_unstable_v1::zriver_options_manager_v1::ZriverOptionsManagerV1,
    river_status_unstable_v1::zriver_status_manager_v1::ZriverStatusManagerV1,
};
use crate::wayland::{
    river_layout_unstable_v1::zriver_layout_v1, river_options_unstable_v1::zriver_option_handle_v1,
    river_status_unstable_v1::zriver_output_status_v1,
};
use wayland_client::protocol::wl_output::WlOutput;
use wayland_client::{DispatchData, Main};

pub struct Globals {
    pub layout_manager: Option<Main<ZriverLayoutManagerV1>>,
    pub options_manager: Option<Main<ZriverOptionsManagerV1>>,
    pub status_manager: Option<Main<ZriverStatusManagerV1>>,
}

pub struct Context {
    pub debug: bool,
    pub running: bool,
    pub namespace: String,
    pub outputs: Vec<Output>,
    pub globals: Globals,
}

pub struct Output {
    pub options: Options,
    pub configured: bool,
    pub output: Option<WlOutput>,
    pub tags: [Option<Tag>; 32],
}

#[derive(Clone, Debug)]
pub struct Tag {
    pub output: Layout,
    pub frames: Vec<Layout>,
}

#[derive(Copy, Clone, Debug)]
pub struct Rectangle {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

#[derive(Clone, Debug)]
pub struct Frame {
    pub parent: bool,
    pub layout: Layout,
    pub rect: Rectangle,
    pub rect_list: Vec<Rectangle>,
}

#[derive(Clone)]
struct Window {
    pub app_id: String,
    pub rect: Rectangle,
}

union Value {
    double: f64,
    uint: u32,
}

impl Context {
    pub fn new() -> Context {
        return {
            Context {
                debug: false,
                running: false,
                namespace: String::from("kile"),
                globals: {
                    Globals {
                        layout_manager: None,
                        options_manager: None,
                        status_manager: None,
                    }
                },
                outputs: Vec::new(),
            }
        };
    }
    pub fn init(&mut self, monitor_index: usize) {
        let output=&mut self.outputs[monitor_index];
        if !output.configured {
            if self.debug {
                output.options.debug();
            }
            output.configure(&self.globals, self.namespace.clone());
        } else {
            self.destroy();
        }
    }
    pub fn destroy(&mut self) {
        self.globals.status_manager.as_ref().unwrap().destroy();
        self.globals.layout_manager.as_ref().unwrap().destroy();
        self.globals.options_manager.as_ref().unwrap().destroy();
        for output in &self.outputs {
            output.destroy();
        }
    }
}

impl Output {
    pub fn new(output: WlOutput) -> Output {
        {
            Output {
                options: Options::new(),
                configured: false,
                output: Some(output),
                tags: Default::default(),
            }
        }
    }
    pub fn configure(&mut self, globals: &Globals, namespace: String) {
        self.get_layout(globals, namespace);
        self.get_tag(globals);
        self.get_option("main_factor", globals);
        self.get_option("main_amount", globals);
        self.get_option("main_index", globals);
        self.get_option("view_padding", globals);
        self.get_option("outer_padding", globals);
        self.get_option("smart_padding", globals);
        self.get_option("output_layout", globals);
        self.get_option("frames_layout", globals);
        self.get_option("layout_per_tag", globals);
        self.configured = true;
    }
    pub fn destroy(&self) {
        (self.options.zlayout).as_ref().unwrap().destroy();
    }
    pub fn update(&mut self) {
        if self.options.view_amount > 0 {
            match self.focused() {
                Some(mut tag) => tag.update(&self.options),
                None => {
                    let mut tag = self.options.default_layout.clone();
                    tag.update(&self.options);
                }
            };
        }
    }
    pub fn set_focused(&mut self, tag: Tag) {
        self.tags[self.options.tagmask as usize] = Some(tag);
    }
    pub fn focused(&self) -> Option<Tag> {
        self.tags[self.options.tagmask as usize].clone()
    }
    pub fn get_layout(&mut self, globals: &Globals, namespace: String) {
        self.options.zlayout = Some(
            globals
                .layout_manager
                .as_ref()
                .expect("Compositor doesn't implement river_layout_unstable_v1")
                .get_river_layout(self.output.as_mut().unwrap(), namespace),
        );
        self.options.zlayout.as_ref().unwrap().quick_assign(
            move |_, event, mut output: DispatchData| match event {
                zriver_layout_v1::Event::LayoutDemand {
                    view_amount,
                    usable_width,
                    usable_height,
                    serial,
                } => {
                    output.get::<Output>().unwrap().options.serial = serial;
                    output.get::<Output>().unwrap().options.view_amount = view_amount;
                    output.get::<Output>().unwrap().options.usable_height = usable_height;
                    output.get::<Output>().unwrap().options.usable_width = usable_width;
                }
                zriver_layout_v1::Event::AdvertiseView {
                    tags,
                    app_id,
                    serial,
                } => {}
                zriver_layout_v1::Event::NamespaceInUse => {
                    println!("Namespace already in use.");
                }
                zriver_layout_v1::Event::AdvertiseDone { serial } => {}
            },
        );
    }
    fn get_tag(&mut self, globals: &Globals) {
        let output_status = globals
            .status_manager
            .as_ref()
            .expect("Compositor doesn't implement river_status_unstable_v1")
            .get_river_output_status(self.output.as_mut().unwrap());
        output_status.quick_assign(move |_, event, mut output| match event {
            zriver_output_status_v1::Event::FocusedTags { tags } => {
                output.get::<Output>().unwrap().options.tagmask = tag_index(tags);
            }
            zriver_output_status_v1::Event::ViewTags { tags } => {}
        });
    }
    fn get_option(&mut self, name: &'static str, globals: &Globals) {
        let option_handle = globals
            .options_manager
            .as_ref()
            .expect("Compositor doesn't implement river_options_unstable_v1")
            .get_option_handle(name.to_owned(), Some(self.output.as_mut().unwrap()));
        option_handle.quick_assign(move |option_handle, event, mut output| {
            let mut option_value: Value = Value { uint: 1 };
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
            unsafe {
                match name {
                    "main_index" => {
                        output.get::<Output>().unwrap().options.main_index = option_value.uint
                    }
                    "main_amount" => {
                        output.get::<Output>().unwrap().options.main_amount = option_value.uint
                    }
                    "main_factor" => {
                        output.get::<Output>().unwrap().options.main_factor = option_value.double
                    }
                    "view_padding" => {
                        output.get::<Output>().unwrap().options.view_padding = option_value.uint
                    }
                    "smart_padding" => match option_value.uint {
                        1 => output.get::<Output>().unwrap().options.smart_padding = true,
                        _ => {}
                    },
                    "outer_padding" => {
                        output.get::<Output>().unwrap().options.outer_padding = option_value.uint
                    }
                    "output_layout" => {
                        match string.as_ref() {
                            "" => {
                                string = String::from("f");
                                option_handle.set_string_value(Some(string.clone()));
                            }
                            _ => {}
                        }
                        output
                            .get::<Output>()
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
                        output
                            .get::<Output>()
                            .unwrap()
                            .options
                            .default_layout
                            .frames = Options::layout_frames(string);
                    }
                    "layout_per_tag" => {
                        output.get::<Output>().unwrap().parse_layout_per_tag(string);
                    }
                    _ => {}
                }
            }
            output
                .get::<Output>()
                .unwrap()
                .options
                .zlayout
                .as_ref()
                .unwrap()
                .parameters_changed();
        });
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
}

pub fn tag_index(mut tagmask: u32) -> u32 {
    let mut tag = 0;
    while tagmask / 2 >= 1 {
        tagmask /= 2;
        tag += 1;
    }
    tag
}

impl Tag {
    pub fn new() -> Tag {
        {
            Tag {
                output: Layout::Full,
                frames: Vec::new(),
            }
        }
    }
    pub fn update(&mut self, options: &Options) {
        let frames_amount = options.frames_amount(self.frames.len() as u32);

        let mut rect = Rectangle::from(options);
        if !options.smart_padding || options.view_amount > 1 {
            rect.apply_padding(options.outer_padding);
        }
        let mut output = Frame::from(self.output,rect, true);
        output.generate(options, frames_amount);

        let main_amount = options.main_amount(frames_amount);
        let slave_amount;

        let mut reste = if main_amount > 0 {
            output.zoom(options.main_index);
            slave_amount = (options.view_amount - main_amount) / (frames_amount - 1);
            (options.view_amount - main_amount) % (frames_amount - 1)
        } else {
            slave_amount = options.view_amount / frames_amount;
            options.view_amount % frames_amount
        };

        let mut main=false;
        for rect in output.rect_list {
            let mut iter = self.frames.iter();
            let layout = match iter.next() {
                Some(layout) => layout,
                None => &Layout::Full,
            };
            let client_count;
            let mut frame=if main && main_amount > 0 {
                main=true;
                client_count=main_amount;
                Frame::from(*layout, rect, false)
            } else {
                client_count = if reste > 0 {
                    reste -= 1;
                    slave_amount + 1
                } else {
                    slave_amount
                };
                Frame::from(*layout, rect, false)
            };
            frame.generate(options, client_count);
            frame.push_dimensions(options);
        }

        options.commit();
    }
}

impl Rectangle {
    pub fn from(options: &Options) -> Rectangle {
        {
            Rectangle {
                x: 0,
                y: 0,
                w: options.usable_width,
                h: options.usable_height,
            }
        }
    }
    pub fn apply_padding(&mut self, padding: u32) {
        if 2 * padding < self.h && 2 * padding < self.w {
            self.x += padding;
            self.y += padding;
            self.w -= 2 * padding;
            self.h -= 2 * padding;
        }
    }
}

impl Frame {
    pub fn from(layout: Layout, rect: Rectangle, parent: bool) -> Frame {
        {
            Frame {
                parent: parent,
                layout: layout,
                rect: rect,
                rect_list: Vec::new(),
            }
        }
    }
    pub fn zoom(&mut self, index: u32) {
        if (index as usize) < self.rect_list.len() {
            let main = self.rect_list[index as usize];
            for i in (0..index as usize).rev() {
                self.rect_list[i + 1] = self.rect_list[i];
            }
            self.rect_list[0] = main;
        }
    }
    pub fn push_dimensions(&mut self, options: &Options) {
        for window in &mut self.rect_list {
            if !options.smart_padding || options.view_amount > 1 {
                window.apply_padding(options.view_padding);
            }
            options.push_dimensions(&window);
        }
    }
    pub fn generate(&mut self, options: &Options, client_count: u32) {
        let mut rect = self.rect;

        if client_count > 0 {
            match self.layout {
                Layout::Tab => {
                    for _i in 0..client_count {
                        self.rect_list.push(rect);
                        rect.h -= 50;
                        rect.y += 50;
                    }
                }
                Layout::Horizontal => {
                    let mut slave_area = rect;
                    let mut main_area = rect;
                    let reste = rect.h % client_count;
                    if self.parent {
                        main_area.h = if options.main_amount > 0
                            && client_count > 1
                            && options.main_amount < options.view_amount
                            && options.main_index < client_count
                        {
                            (rect.h * (options.main_factor * 100.0) as u32)
                                / (50 * client_count)
                        } else {
                            0
                        };
                        slave_area.h -= main_area.h;
                    }
                    for i in 0..client_count {
                        if self.parent && i == options.main_index && main_area.h > 0 {
                            rect.h = main_area.h;
                        } else if self.parent && main_area.h > 0 {
                            rect.h = slave_area.h / (client_count - 1);
                        } else {
                            rect.h = slave_area.h / client_count;
                        }
                        if i == 0 {
                            rect.h += reste;
                        }

                        self.rect_list.push(rect);
                        rect.y += rect.h;
                    }
                }
                Layout::Vertical => {
                    let mut slave_area = rect;
                    let mut main_area = rect;
                    let reste = rect.w % client_count;
                    if self.parent {
                        main_area.w = if options.main_amount > 0
                            && client_count > 1
                            && options.main_amount < options.view_amount
                            && options.main_index < client_count
                        {
                            (rect.w * (options.main_factor * 100.0) as u32)
                                / (50 * client_count)
                        } else {
                            0
                        };
                        slave_area.w -= main_area.w;
                    }
                    for i in 0..client_count {
                        if self.parent && i == options.main_index && main_area.w > 0 {
                            rect.w = main_area.w;
                        } else if self.parent && main_area.w > 0 {
                            rect.w = slave_area.w / (client_count - 1);
                        } else {
                            rect.w = slave_area.w / client_count;
                        }
                        if i == 0 {
                            rect.w += reste;
                        }

                        self.rect_list.push(rect);
                        rect.x += rect.w;
                    }
                }
                Layout::Full => {
                    for _i in 0..client_count {
                        self.rect_list.push(rect);
                    }
                }
            }
        }
    }
}
