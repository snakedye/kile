use super::layout::Layout;
use super::parser;
use crate::wayland::{
    river_layout_v2::river_layout_manager_v2::RiverLayoutManagerV2,
    river_layout_v2::river_layout_v2::Event,
};
use wayland_client::protocol::wl_output::WlOutput;
use wayland_client::Main;

pub struct Context {
    pub outputs: Vec<Output>,
    pub layout_manager: Option<Main<RiverLayoutManagerV2>>,
}

pub struct Options {
    pub windows: Vec<Window>,
    pub main_amount: u32,
    pub main_index: u32,
    pub main_factor: f64,
}

pub struct Output {
    pub default: Tag,
    pub resized: bool,
    pub focused: usize,
    pub output: WlOutput,
    pub dimension: Area,
    pub smart_padding: bool,
    pub outer_padding: u32,
    pub view_padding: u32,
    pub tags: [Option<Tag>; 32],
}

pub struct Tag {
    pub rule: Rule,
    pub outer: Layout,
    pub options: Options,
    pub inner: Vec<Layout>,
}

pub struct Window {
    pub tags: u32,
    pub app_id: String,
    pub area: Option<Area>,
}

pub struct Frame {
    pub area: Area,
    pub list: Vec<Rectangle>,
}

#[derive(Copy, Clone)]
pub struct Area {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

#[derive(Clone, Debug)]
pub enum Rule {
    AppId(String),
    Tag(u32),
    None,
}

pub enum Rectangle {
    Area(Area),
    Window(Window),
}

impl Context {
    pub fn new() -> Context {
        return {
            Context {
                layout_manager: None,
                outputs: Vec::new(),
            }
        };
    }
}

impl Options {
    pub fn new() -> Options {
        return {
            Options {
                windows: Vec::new(),
                main_factor: 0.55,
                main_index: 0,
                main_amount: 1,
            }
        };
    }
}

impl Output {
    pub fn new(output: WlOutput) -> Output {
        {
            Output {
                output: output,
                dimension: Area {
                    x: 0,
                    y: 0,
                    w: 0,
                    h: 0,
                },
                focused: 0,
                view_padding: 0,
                outer_padding: 0,
                resized: false,
                smart_padding: false,
                tags: Default::default(),
                default: {
                    Tag {
                        options: Options::new(),
                        rule: Rule::None,
                        outer: Layout::Full,
                        inner: vec![Layout::Full],
                    }
                },
            }
        }
    }
    pub fn layout_filter(
        mut self,
        layout_manager: Option<&Main<RiverLayoutManagerV2>>,
        namespace: String,
    ) {
        let layout = layout_manager
            .expect("Compositor doesn't implement river_layout_v2")
            .get_layout(&self.output, namespace);
        layout.quick_assign(move |layout, event, _| match event {
            Event::LayoutDemand {
                view_count: _,
                usable_width,
                usable_height,
                serial: _,
                mut tags,
            } => {
                if !self.resized {
                    self.dimension = Area::from(0, 0, usable_width, usable_height);
                }
                self.focused = {
                    let mut i = 0;
                    while tags > 1 {
                        tags /= 2;
                        i += 1;
                    }
                    i as usize
                };
            }
            Event::AdvertiseView {
                tags,
                app_id,
                serial: _,
            } => {
                self.default.options.windows.push(Window {
                    app_id: app_id.unwrap(),
                    area: None,
                    tags: tags,
                });
            }
            Event::NamespaceInUse => {
                println!("Namespace already in use.");
            }
            Event::AdvertiseDone { serial } => {
                self.dimension.apply_padding(self.outer_padding);
                let mut list = match self.tags[self.focused].as_mut() {
                    Some(tag) => {
                        tag.options
                            .windows
                            .append(&mut self.default.options.windows);
                        tag.update(Rectangle::Area(self.dimension))
                    }
                    None => self.default.update(Rectangle::Area(self.dimension)),
                };
                let view_amount = list.len();
                for rect in &mut list {
                    if !self.smart_padding || view_amount > 1 {
                        rect.apply_padding(self.view_padding);
                    }
                    let rect = rect.area();
                    layout.push_view_dimensions(
                        serial,
                        rect.x as i32,
                        rect.y as i32,
                        rect.w,
                        rect.h,
                    )
                }
                layout.commit(serial);
            }
            Event::SetIntValue { name, mut value } => match name.as_ref() {
                "main_amount" | "main_index" => {
                    if let Some(tag) = self.tags[self.focused].as_mut() {
                        if value >= 0 {
                            match name.as_ref() {
                                "main_amount" => tag.options.main_amount = value as u32,
                                "main_index" => tag.options.main_index = value as u32,
                                _ => {}
                            }
                        }
                    }
                }
                "view_padding" => self.view_padding = value as u32,
                "outer_padding" => self.outer_padding = value as u32,
                "xoffset" => {
                    if value != 0 || value < self.dimension.x as i32 {
                        if value < 0 {
                            self.dimension.x = 0;
                            value = value * (-1);
                        } else {
                            self.dimension.x = value as u32;
                        }
                        self.dimension.w -= value as u32;
                        self.resized = true;
                    } else {
                        self.resized = false;
                    }
                }
                "yoffset" => {
                    if value != 0 || value < self.dimension.y as i32 {
                        if value < 0 {
                            self.dimension.y = 0;
                            value = value * (-1);
                        } else {
                            self.dimension.y = value as u32;
                        }
                        self.dimension.h -= value as u32;
                        self.resized = true;
                    } else {
                        self.resized = false;
                    }
                }
                _ => {}
            },
            Event::ModIntValue { name, delta } => match name.as_ref() {
                "main_amount" | "main_index" => {
                    if let Some(tag) = self.tags[self.focused].as_mut() {
                        match name.as_ref() {
                            "main_amount" => {
                                tag.options.main_amount =
                                    ((tag.options.main_amount as i32) + delta) as u32
                            }
                            "main_index" => {
                                tag.options.main_index =
                                    ((tag.options.main_index as i32) + delta) as u32
                            }
                            _ => {}
                        }
                    }
                }
                "view_padding" => self.view_padding = ((self.view_padding as i32) + delta) as u32,
                "outer_padding" => {
                    self.outer_padding = ((self.outer_padding as i32) + delta) as u32
                }
                _ => {}
            },
            Event::SetFixedValue { name, value } => {
                if name == "main_factor" {
                    if let Some(tag) = self.tags[self.focused].as_mut() {
                        if value > 0.0 && value < 1.0 {
                            tag.options.main_factor = value
                        }
                    }
                }
            }
            Event::ModFixedValue { name, delta } => {
                if name == "main_factor" {
                    if let Some(tag) = self.tags[self.focused].as_mut() {
                        if delta <= tag.options.main_factor {
                            tag.options.main_factor += delta;
                        }
                    }
                }
            }
            Event::SetStringValue { name, value } => parser::main(&mut self, name, value),
        });
    }
}

impl Window {
    pub fn apply_padding(&mut self, padding: u32) {
        self.area.as_mut().unwrap().apply_padding(padding);
    }
    pub fn compare(&self, rule: &Rule) -> bool {
        match rule {
            Rule::AppId(string) => string.eq(&self.app_id),
            Rule::Tag(uint) => self.tags == *uint,
            _ => false,
        }
    }
}

impl Area {
    pub fn from(x: u32, y: u32, w: u32, h: u32) -> Area {
        {
            Area {
                x: x,
                y: y,
                w: w,
                h: h,
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

impl Tag {
    pub fn update(&mut self, mut area: Rectangle) -> Vec<Rectangle> {
        let view_amount = self.options.windows.len() as u32;
        let slave_amount;
        let frames_available = self.inner.len() as u32;
        let frame_amount = if self.options.main_amount >= view_amount {
            1
        } else if view_amount >= frames_available {
            if 1 + view_amount - self.options.main_amount < frames_available
                && self.options.main_amount > view_amount / frames_available
            {
                1 + view_amount - self.options.main_amount
            } else {
                frames_available
            }
        } else {
            view_amount
        };
        self.options.main_amount = if self.options.main_index + self.options.main_amount
            <= view_amount
            && frame_amount > 1
            && self.options.main_amount > 0
        {
            if self.options.main_index + self.options.main_amount > view_amount {
                view_amount - self.options.main_index
            } else {
                self.options.main_amount
            }
        } else {
            0
        };
        let mut list = area.generate(&mut self.options, frame_amount, self.outer, true, true);
        let mut reste = if self.options.main_amount > 0 {
            zoom(&mut list, self.options.main_index as usize);
            slave_amount = (view_amount - self.options.main_amount) / (frame_amount - 1);
            (view_amount - self.options.main_amount) % (frame_amount - 1)
        } else {
            slave_amount = view_amount / frame_amount;
            view_amount % frame_amount
        };

        let mut windows = Vec::new();
        for (i, rect) in list.iter_mut().enumerate() {
            let mut list = {
                let amount = if i == 0 && self.options.main_amount != 0 {
                    self.options.main_amount
                } else {
                    if reste > 0 {
                        reste -= 1;
                        slave_amount + 1
                    } else {
                        slave_amount
                    }
                };
                rect.generate(&mut self.options, amount, self.inner[i], false, false)
            };
            windows.append(&mut list);
        }
        self.focus_all(&mut windows);
        windows
    }
    pub fn focus_all(&self, list: &mut Vec<Rectangle>) {
        let mut i = list.len() - 1;
        let mut zoomed = 0;
        let mut to = 0;
        while to < i && list[to].compare(&self.rule) {
            to += 1;
        }
        while i > 0 {
            let mut j = i;
            while j > to && zoomed < self.options.main_amount && list[i].compare(&self.rule) {
                focus(list, i, to);
                j -= 1;
                zoomed += 1;
            }
            if i != j {
                i = j
            } else {
                i -= 1
            }
        }
    }
}

fn zoom(list: &mut Vec<Rectangle>, index: usize) {
    if (index as usize) < list.len() {
        let area = list[index].area();
        for i in (0..index).rev() {
            let previous = list[i].area();
            list[i + 1].set(previous);
        }
        list[0].set(area);
    }
}
fn focus(list: &mut Vec<Rectangle>, index: usize, to: usize) {
    if (index as usize) < list.len() {
        let main = list[to].area();
        for i in to..index {
            let forward = list[i + 1].area();
            list[i].set(forward);
        }
        list[index].set(main);
    }
}
