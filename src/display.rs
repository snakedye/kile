use super::options::{Layout, Options};
use crate::wayland::{
    river_layout_unstable_v1::zriver_layout_manager_v1::ZriverLayoutManagerV1,
    river_options_unstable_v1::zriver_options_manager_v1::ZriverOptionsManagerV1,
};
use crate::wayland::{
    river_layout_unstable_v1::zriver_layout_v1, river_options_unstable_v1::zriver_option_handle_v1,
};
use wayland_client::protocol::wl_output::WlOutput;
use wayland_client::{DispatchData, Main};

pub struct Globals {
    pub layout_manager: Option<Main<ZriverLayoutManagerV1>>,
    pub options_manager: Option<Main<ZriverOptionsManagerV1>>,
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
    pub default_layout: Tag,
    pub output: Option<WlOutput>,
    pub tags: [Option<Tag>; 32],
}

#[derive(Clone)]
pub struct Tag {
    // titlebar: Titlebar,
    pub outer: Layout,
    pub inner: Vec<Layout>,
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
    // fn titlebar(&self);
    fn apply_padding(&mut self, padding: u32);
    fn push_dimensions(&mut self, options: &Options);
    // fn new() -> R;
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

impl Output {
    pub fn new(output: WlOutput) -> Output {
        {
            Output {
                options: Options::new(),
                default_layout: Tag::new(),
                configured: false,
                output: Some(output),
                tags: Default::default(),
            }
        }
    }
    pub fn configure(&mut self, globals: &Globals, namespace: String) {
        self.get_layout(globals, namespace);
        self.get_option("main_factor", globals);
        self.get_option("main_amount", globals);
        self.get_option("main_index", globals);
        self.get_option("view_padding", globals);
        self.get_option("outer_padding", globals);
        self.get_option("xoffset", globals);
        self.get_option("yoffset", globals);
        self.get_option("smart_padding", globals);
        self.get_option("command", globals);
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
                None => self.default_layout.update(&mut self.options),
            }
        }
    }
    fn get_layout(&mut self, globals: &Globals, namespace: String) {
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
                        i
                    }
                }
                zriver_layout_v1::Event::AdvertiseView {
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
                zriver_layout_v1::Event::NamespaceInUse => {
                    println!("Namespace already in use.");
                }
                zriver_layout_v1::Event::AdvertiseDone { serial } => {}
            },
        );
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
                    string = value.unwrap_or_default();
                }
                zriver_option_handle_v1::Event::FixedValue { value } => option_value.double = value,
                zriver_option_handle_v1::Event::UintValue { value } => option_value.uint = value,
                zriver_option_handle_v1::Event::IntValue { value } => option_value.int = value,
                zriver_option_handle_v1::Event::Unset => {}
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
                    "command" => {
                        let mut command = string.split_whitespace();
                        match command.next().unwrap_or_default() {
                            "smart-padding" => match command.next() {
                                Some(arg) => match arg.parse::<bool>() {
                                    Ok(ans) => output_handle.options.smart_padding = ans,
                                    Err(_) => {}
                                },
                                None => {}
                            },
                            "set-layout" => {
                                for arg in command {
                                    output_handle.parse_tag_config(arg.to_string())
                                }
                            }
                            "clear-tag" => match command.next() {
                                Some(arg) => match arg {
                                    "all" => output_handle.tags = Default::default(),
                                    _ => match arg.parse::<usize>() {
                                        Ok(int) => output_handle.tags[int] = None,
                                        Err(_) => {}
                                    },
                                },
                                None => {}
                            },
                            _ => {}
                        }
                        option_handle.set_string_value(Some("".to_string()));
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
    fn parse_tag_config(&mut self, layout_per_tag: String) {
        let mut layout_per_tag = layout_per_tag.split_whitespace();
        loop {
            match layout_per_tag.next() {
                Some(rule) => {
                    let mut rule = rule.split(':');
                    let tags = match rule.next() {
                        Some(tag) => match tag {
                            "focused" => self.options.tagmask..self.options.tagmask+1,
                            "all" => 0..32,
                            _ => match tag.parse::<u32>() {
                                Ok(int) => {
                                    if int > 0 && int < 32 {
                                        int..int+1
                                    } else {
                                        break;
                                    }
                                }
                                Err(e) => {
                                    println!("{}", e);
                                    break;
                                }
                            },
                        },
                        None => {
                            break;
                        }
                    };
                    let outer_layout =
                        Options::outer_layout(rule.next().unwrap_or_default().to_string());
                    let inner_layout =
                        Options::inner_layout(rule.next().unwrap_or("f").to_string());
                    for i in tags {
                        self.tags[i as usize] = Some({
                            Tag {
                                // titlebar: None,
                                outer: outer_layout,
                                inner: inner_layout.clone(),
                                frame: None,
                            }
                        });
                    }
                }
                None => break,
            }
        }
    }
}

impl Tag {
    pub fn new() -> Tag {
        {
            Tag {
                // titlebar: None,
                outer: Layout::Full,
                inner: vec![Layout::Full],
                frame: None,
            }
        }
    }
    fn push_views(&mut self, options: &Options) {
        let frame = self.frame.as_mut().unwrap();
        for i in 0..frame.list.len() {
            match frame.list[i].app_id.as_ref() {
                "firefox" => {
                    frame.focus(i);
                    break;
                }
                _ => {}
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
        outer_frame.generate(frame_amount, options, true);
        let main_amount = options.main_amount(frame_amount);
        let slave_amount;
        let mut reste = if main_amount > 0 {
            outer_frame.focus(options.main_index as usize);
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
                frame.generate(main_amount, options, false);
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
                );
            }
            windows.append(&mut frame.list);
        }
        outer_frame.list = windows;

        self.push_views(options);
    }
}

impl Rectangle for Window {
    fn apply_padding(&mut self, padding: u32) {
        let mut area = self.area.unwrap();
        area.apply_padding(padding);
        self.area = Some(area);
    }
    fn push_dimensions(&mut self, options: &Options) {
        if !options.smart_padding || options.view_amount > 1 {
            self.apply_padding(options.view_padding);
        }
        options.push_dimensions(&self.area.unwrap());
    }
}

impl Rectangle for Area {
    fn apply_padding(&mut self, padding: u32) {
        if 2 * padding < self.h && 2 * padding < self.w {
            self.x += padding;
            self.y += padding;
            self.w -= 2 * padding;
            self.h -= 2 * padding;
        }
    }
    fn push_dimensions(&mut self, options: &Options) {
        options.push_dimensions(self);
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
                    tags: 0,
                }
            }
        };
        window.area = Some(area);
        self.list.push(window);
    }
    pub fn generate(&mut self, client_count: u32, options: &mut Options, parent: bool) {
        let mut area = self.area;

        match self.layout {
            Layout::Tab => {
                for _i in 0..client_count {
                    self.insert_window(area, options, parent);
                    area.h -= 50;
                    area.y += 50;
                }
                // self.area.titlebar(self.list);
            }
            Layout::Horizontal => {
                let mut slave_area = area;
                let mut main_area = area;
                let reste = area.h % client_count;
                if parent {
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
                    if parent && i == options.main_index && main_area.h > 0 {
                        area.h = main_area.h;
                    } else if parent && main_area.h > 0 {
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
                if parent {
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
                    if parent && i == options.main_index && main_area.w > 0 {
                        area.w = main_area.w;
                    } else if parent && main_area.w > 0 {
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
            Layout::Recursive { modi, index } => {
                for i in 0..client_count {
                    let mut frame = Frame::new(
                        if (i + modi) % 2 == 0 {
                            Layout::Vertical
                        } else {
                            Layout::Horizontal
                        },
                        area,
                    );
                    if i < client_count - 1 {
                        frame.generate(2, options, true);
                        match index {
                            Some(index) => {
                                if index as u32 > i {
                                    self.insert_window(frame.list[0].area.unwrap(), options, false)
                                } else {
                                    self.insert_window(
                                        frame.list[index].area.unwrap(),
                                        options,
                                        false,
                                    )
                                }
                            }
                            None => self.insert_window(frame.list[0].area.unwrap(), options, false),
                        }
                        area = frame.list.last().unwrap().area.unwrap();
                    } else {
                        frame.generate(1, options, false);
                        self.insert_window(frame.list[0].area.unwrap(), options, false);
                    }
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
