use super::options::{Layout, Options};
use crate::wayland::{
    river_layout_v1::river_layout_manager_v1::RiverLayoutManagerV1,
    river_options_v2::river_options_manager_v2::RiverOptionsManagerV2,
};
use crate::wayland::{river_layout_v1::river_layout_v1, river_options_v2::river_option_handle_v2};
use wayland_client::protocol::wl_output::WlOutput;
use wayland_client::{DispatchData, Main};

pub struct Globals {
    pub layout_manager: Option<Main<RiverLayoutManagerV1>>,
    pub options_manager: Option<Main<RiverOptionsManagerV2>>,
}

pub struct Context {
    pub running: bool,
    pub namespace: String,
    pub outputs: Vec<Output>,
    pub globals: Globals,
}

pub struct Output {
    pub options: Options,
    pub configured: bool,
    pub default: Tag,
    pub output: Option<WlOutput>,
    pub tags: [Option<Tag>; 32],
}

#[derive(Clone)]
pub struct Tag {
    pub outer: Layout,
    pub inner: Vec<Layout>,
    pub preferred_app: Option<String>,
    pub frame: Option<Frame>,
}

#[derive(Clone, Debug)]
pub struct Window {
    pub area: Option<Area>,
    pub app_id: String,
    pub tags: u32,
}

#[derive(Clone)]
pub struct Frame {
    pub layout: Layout,
    pub area: Area,
    pub list: Vec<Window>,
}

#[derive(Copy, Clone, Debug)]
pub struct Area {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

union Value {
    double: f64,
    uint: u32,
    int: i32,
}

pub trait Rectangle {
    fn apply_padding(&mut self, padding: u32);
    fn push_dimensions(&mut self, options: &Options);
}

impl Context {
    pub fn new() -> Context {
        return {
            Context {
                running: false,
                namespace: String::from("kile"),
                globals: {
                    Globals {
                        layout_manager: None,
                        options_manager: None,
                    }
                },
                outputs: Vec::new(),
            }
        };
    }
    pub fn init(&mut self, monitor_index: usize) {
        self.globals.declare_uint_option("view_padding", 10);
        self.globals.declare_uint_option("outer_padding", 5);
        self.globals.declare_uint_option("main_index", 0);
        self.globals.declare_uint_option("main_amount", 1);
        self.globals.declare_int_option("xoffset", 0);
        self.globals.declare_int_option("yoffset", 0);
        self.globals.declare_fixed_option("main_factor", 0.6);
        self.globals.declare_string_option("command", None);

        let output = &mut self.outputs[monitor_index];
        if !output.configured {
            output.configure(&self.globals, self.namespace.clone());
        } else {
            self.destroy();
        }
    }
    pub fn destroy(&mut self) {
        self.globals.layout_manager.as_ref().unwrap().destroy();
        self.globals.options_manager.as_ref().unwrap().destroy();
        for output in &self.outputs {
            output.destroy();
        }
    }
}

impl Globals {
    pub fn declare_uint_option(&self, name: &'static str, value: u32) {
        self.options_manager
            .as_ref()
            .expect("Compositor doesn't implement river_options_v2")
            .declare_uint_option(name.to_owned(), value);
    }
    pub fn declare_int_option(&self, name: &'static str, value: i32) {
        self.options_manager
            .as_ref()
            .expect("Compositor doesn't implement river_options_v2")
            .declare_int_option(name.to_owned(), value);
    }
    pub fn declare_string_option(&self, name: &'static str, value: Option<String>) {
        self.options_manager
            .as_ref()
            .expect("Compositor doesn't implement river_options_v2")
            .declare_string_option(name.to_owned(), value);
    }
    pub fn declare_fixed_option(&self, name: &'static str, value: f64) {
        self.options_manager
            .as_ref()
            .expect("Compositor doesn't implement river_options_v2")
            .declare_fixed_option(name.to_owned(), value);
    }
    pub fn get_layout(&self, output: &mut Output, namespace: String) {
        output.options.zlayout = Some(
            self.layout_manager
                .as_ref()
                .expect("Compositor doesn't implement river_layout_v1")
                .get_river_layout(output.output.as_ref().unwrap(), namespace),
        );
        output.options.zlayout.as_ref().unwrap().quick_assign(
            move |_, event, mut output: DispatchData| match event {
                river_layout_v1::Event::LayoutDemand {
                    view_amount,
                    usable_width,
                    usable_height,
                    serial,
                    mut tags,
                } => {
                    output.get::<Output>().unwrap().options.serial = serial;
                    output.get::<Output>().unwrap().options.view_amount = view_amount;
                    output.get::<Output>().unwrap().options.usable_height = usable_height;
                    output.get::<Output>().unwrap().options.usable_width = usable_width;
                    output.get::<Output>().unwrap().options.tagmask = {
                        let mut i = 0;
                        while tags / 2 >= 1 {
                            tags /= 2;
                            i += 1;
                        }
                        i as usize
                    };
                }
                river_layout_v1::Event::AdvertiseView {
                    tags,
                    app_id,
                    serial,
                } => {
                    output
                        .get::<Output>()
                        .unwrap()
                        .options
                        .windows
                        .push(Window {
                            app_id: app_id.unwrap(),
                            area: None,
                            tags: tags,
                        });
                }
                river_layout_v1::Event::NamespaceInUse => {
                    println!("Namespace already in use.");
                }
                river_layout_v1::Event::AdvertiseDone { serial } => {
                    output.get::<Output>().unwrap().update()
                }
            },
        );
    }
    pub fn get_option(&self, name: &'static str, output: &mut Output) {
        let option_handle = self
            .options_manager
            .as_ref()
            .expect("Compositor doesn't implement river_options_v2")
            .get_option_handle(name.to_owned(), Some(output.output.as_ref().unwrap()));
        option_handle.quick_assign(move |option_handle, event, mut output| {
            let mut option_value: Value = Value { uint: 1 };
            let mut string: Option<String> = None;
            match event {
                river_option_handle_v2::Event::StringValue { value } => string = value,
                river_option_handle_v2::Event::FixedValue { value } => option_value.double = value,
                river_option_handle_v2::Event::UintValue { value } => option_value.uint = value,
                river_option_handle_v2::Event::IntValue { value } => option_value.int = value,
                river_option_handle_v2::Event::Undeclared => {}
            }
            let output_handle = output.get::<Output>().unwrap();
            unsafe {
                match name {
                    "main_index" => output_handle.options.main_index = option_value.uint,
                    "main_amount" => output_handle.options.main_amount = option_value.uint,
                    "main_factor" => output_handle.options.main_factor = option_value.double,
                    "view_padding" => output_handle.options.view_padding = option_value.uint,
                    "xoffset" => output_handle.options.xoffset = option_value.int,
                    "yoffset" => output_handle.options.yoffset = option_value.int,
                    "outer_padding" => output_handle.options.outer_padding = option_value.uint,
                    "command" => match string {
                        Some(command) => {
                            let mut command = command.split_whitespace();
                            match command.next().unwrap_or_default() {
                                "smart-padding" => {
                                    if let Ok(ans) = command.next().unwrap().parse::<bool>() {
                                        output_handle.options.smart_padding = ans;
                                    }
                                }
                                "set-tag" => {
                                    for arg in command {
                                        output_handle.parse_tag_config(arg.to_string())
                                    }
                                }
                                "preferred-app" => {
                                    output_handle.tags[output_handle.options.tagmask as usize]
                                        .as_mut()
                                        .unwrap()
                                        .preferred_app =
                                        Some(command.map(|app_id| app_id.to_string()).collect())
                                }
                                "clear-tag" => match command.next() {
                                    Some(arg) => match arg {
                                        "all" => output_handle.tags = Default::default(),
                                        "focused" => {
                                            output_handle.tags
                                                [output_handle.options.tagmask as usize] = None
                                        }
                                        _ => match arg.parse::<usize>() {
                                            Ok(int) => output_handle.tags[int] = None,
                                            Err(_) => {}
                                        },
                                    },
                                    None => {}
                                },
                                _ => {}
                            }
                            option_handle.set_string_value(None);
                        }
                        None => {}
                    },
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
}

impl Output {
    pub fn new(output: WlOutput) -> Output {
        {
            Output {
                options: Options::new(),
                default: Tag::new(),
                configured: false,
                output: Some(output),
                tags: Default::default(),
            }
        }
    }
    pub fn configure(&mut self, globals: &Globals, namespace: String) {
        globals.get_layout(self, namespace);
        globals.get_option("main_factor", self);
        globals.get_option("main_amount", self);
        globals.get_option("main_index", self);
        globals.get_option("view_padding", self);
        globals.get_option("outer_padding", self);
        globals.get_option("xoffset", self);
        globals.get_option("yoffset", self);
        globals.get_option("command", self);
        self.configured = true;
    }
    pub fn destroy(&self) {
        (self.options.zlayout).as_ref().unwrap().destroy();
    }
    pub fn update(&mut self) {
        if self.options.view_amount > 0 {
            let focused = self.tags[self.options.tagmask as usize].as_mut();
            self.options.rearrange();
            match focused {
                Some(tag) => tag.update(&mut self.options),
                None => self.default.update(&mut self.options),
            }
        }
    }
    fn parse_tag_config(&mut self, layout_per_tag: String) {
        let mut layout_per_tag = layout_per_tag.split_whitespace();
        loop {
            match layout_per_tag.next() {
                Some(rule) => {
                    let mut rule = rule.split(':');
                    let tags = match rule.next() {
                        Some(tag) => match tag {
                            "focused" => self.options.tagmask..self.options.tagmask + 1,
                            "all" => 0..32,
                            _ => match tag.parse::<usize>() {
                                Ok(int) => {
                                    if int > 0 && int < 33 {
                                        int - 1..int
                                    } else {
                                        break;
                                    }
                                }
                                Err(_) => break,
                            },
                        },
                        None => {
                            break;
                        }
                    };
                    let outer_layout =
                        Options::outer_layout(rule.next().unwrap_or_default().to_string());
                    let inner_layout =
                        Options::inner_layout(rule.next().unwrap_or_default().to_string());
                    let preferred_app = match rule.next() {
                        Some(app_id) => Some(app_id.to_string()),
                        None => None,
                    };
                    for i in tags {
                        let tag = self.tags[i].as_mut();
                        match tag {
                            Some(tag) => {
                                if let Some(outer_layout) = outer_layout {
                                    tag.outer = outer_layout;
                                }
                                if let Some(inner_layout) = inner_layout.clone() {
                                    tag.inner = inner_layout;
                                }
                                tag.preferred_app = preferred_app.clone();
                            }
                            None => {
                                self.tags[i] = Some({
                                    Tag {
                                        outer: outer_layout.unwrap_or(Layout::Full),
                                        inner: inner_layout.clone().unwrap_or(vec![Layout::Full]),
                                        preferred_app: preferred_app.clone(),
                                        frame: None,
                                    }
                                })
                            }
                        }
                    }
                }
                None => break,
            }
        }
    }
    pub fn debug(&self) {
        match &self.tags[self.options.tagmask] {
            Some(tag) => {
                println!(
                    "Tag - {}\n
                preferred-app: {:?}\n
                inner_layout: {:?}\n
                outer_layout: {:?}\n
                windows: {:?}",
                    self.options.tagmask,
                    tag.preferred_app,
                    tag.inner,
                    tag.outer,
                    self.options.windows
                );
            }
            None => {}
        }
    }
}

impl Tag {
    pub fn new() -> Tag {
        {
            Tag {
                preferred_app: None,
                outer: Layout::Full,
                inner: vec![Layout::Full],
                frame: None,
            }
        }
    }
    fn push_views(&mut self, options: &Options) {
        let frame = self.frame.as_mut().unwrap();
        for i in 0..frame.list.len() {
            if frame.list[i].app_id.eq(&self
                .preferred_app
                .as_ref()
                .unwrap_or(&"".to_string())
                .as_ref())
            {
                frame.focus(i);
                break;
            }
        }
        for window in &mut frame.list {
            window.push_dimensions(options);
        }
        options.commit();
    }
    pub fn update(&mut self, options: &mut Options) {
        self.frame = Some(Frame::new(self.outer, options.get_output()));
        let outer_frame = self.frame.as_mut().unwrap();
        let frame_amount = options.frames_amount(self.inner.len() as u32);
        outer_frame.generate(frame_amount, options, true, true);
        let main_amount = options.main_amount(frame_amount);
        let slave_amount;
        let mut reste = if main_amount > 0 {
            outer_frame.zoom(options.main_index as usize);
            slave_amount = (options.view_amount - main_amount) / (frame_amount - 1);
            (options.view_amount - main_amount) % (frame_amount - 1)
        } else {
            slave_amount = options.view_amount / frame_amount;
            options.view_amount % frame_amount
        };

        let mut windows = Vec::new();
        for (i, window) in outer_frame.list.iter().enumerate() {
            let mut frame = Frame::new(self.inner[i], window.area.unwrap());
            if i == options.main_index as usize && main_amount != 0 as u32 {
                frame.generate(main_amount, options, false, false);
            } else {
                frame.generate(
                    if reste > 0 {
                        reste -= 1;
                        slave_amount + 1
                    } else {
                        slave_amount
                    },
                    options,
                    false,
                    false,
                );
            }
            windows.append(&mut frame.list);
        }
        outer_frame.list = windows;

        self.push_views(options);
    }
}

impl Window {
    pub fn apply_padding(&mut self, padding: u32) {
        let mut area = self.area.unwrap();
        area.apply_padding(padding);
        self.area = Some(area);
    }
    pub fn push_dimensions(&mut self, options: &Options) {
        if !options.smart_padding || options.view_amount > 1 {
            self.apply_padding(options.view_padding);
        }
        options.push_dimensions(&self.area.unwrap());
    }
}

impl Area {
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
    pub fn new(layout: Layout, area: Area) -> Frame {
        {
            Frame {
                layout: layout,
                area: area,
                list: Vec::new(),
            }
        }
    }
    pub fn zoom(&mut self, index: usize) {
        if (index as usize) < self.list.len() {
            let main = self.list[index].area;
            for i in (0..index).rev() {
                self.list[i + 1].area = self.list[i].area;
            }
            self.list[0].area = main;
        }
    }
    pub fn focus(&mut self, index: usize) {
        if (index as usize) < self.list.len() {
            let main = self.list[0].area;
            for i in 0..index {
                self.list[i].area = self.list[i + 1].area;
            }
            self.list[index].area = main;
        }
    }
    fn insert_window(&mut self, area: Area, options: &mut Options, parent: bool) {
        let mut window = if !parent && options.windows.len() > 0 {
            options.windows.remove(0)
        } else {
            {
                Window {
                    app_id: String::new(),
                    area: None,
                    tags: options.tagmask as u32,
                }
            }
        };
        window.area = Some(area);
        self.list.push(window);
    }
    pub fn generate(
        &mut self,
        client_count: u32,
        options: &mut Options,
        parent: bool,
        mut factor: bool,
    ) {
        let mut area = self.area;

        match self.layout {
            Layout::Tab => {
                for _i in 0..client_count {
                    self.insert_window(area, options, parent);
                    area.h -= 50;
                    area.y += 50;
                }
            }
            Layout::Horizontal => {
                let mut slave_area = area;
                let mut main_area = area;
                let reste = area.h % client_count;
                if factor {
                    main_area.h = if options.main_amount > 0
                        && client_count > 1
                        && options.main_amount < options.view_amount
                        && options.main_index < client_count
                    {
                        (area.h * (options.main_factor * 100.0) as u32) / (50 * client_count)
                    } else {
                        0
                    };
                    slave_area.h -= main_area.h;
                }
                for i in 0..client_count {
                    if factor && i == options.main_index && main_area.h > 0 {
                        area.h = main_area.h;
                    } else if factor && main_area.h > 0 {
                        area.h = slave_area.h / (client_count - 1);
                    } else {
                        area.h = slave_area.h / client_count;
                    }
                    if i == 0 {
                        area.h += reste;
                    }

                    self.insert_window(area, options, parent);
                    area.y += area.h;
                }
            }
            Layout::Vertical => {
                let mut slave_area = area;
                let mut main_area = area;
                let reste = area.w % client_count;
                if factor {
                    main_area.w = if options.main_amount > 0
                        && client_count > 1
                        && options.main_amount < options.view_amount
                        && options.main_index < client_count
                    {
                        (area.w * (options.main_factor * 100.0) as u32) / (50 * client_count)
                    } else {
                        0
                    };
                    slave_area.w -= main_area.w;
                }
                for i in 0..client_count {
                    if factor && i == options.main_index && main_area.w > 0 {
                        area.w = main_area.w;
                    } else if factor && main_area.w > 0 {
                        area.w = slave_area.w / (client_count - 1);
                    } else {
                        area.w = slave_area.w / client_count;
                    }
                    if i == 0 {
                        area.w += reste;
                    }

                    self.insert_window(area, options, parent);
                    area.x += area.w;
                }
            }
            Layout::Recursive { modi } => {
                for i in 0..client_count {
                    self.layout = if (i + modi) % 2 == 0 {
                        Layout::Vertical
                    } else {
                        Layout::Horizontal
                    };
                    if i < client_count - 1 {
                        self.generate(2, options, true, factor);
                        let index = self.list.len() - 1;
                        self.area = self.list.remove(index).area.unwrap();
                        if !parent && options.windows.len() > 0 {
                            let mut window = options.windows.remove(0);
                            window.area = self.list[index - 1].area;
                            self.list[index - 1] = window;
                        }
                    } else {
                        self.generate(1, options, parent, factor);
                    }
                    factor = false;
                }
            }
            Layout::Full => {
                for _i in 0..client_count {
                    self.insert_window(area, options, parent);
                }
            }
        }
    }
}
