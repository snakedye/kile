use super::layout::Layout;
use super::parser;
use crate::wayland::{
    river_layout_v2::river_layout_manager_v2::RiverLayoutManagerV2,
    river_layout_v2::river_layout_v2::{Event, RiverLayoutV2},
};
use wayland_client::protocol::wl_output::WlOutput;
use wayland_client::Main;

pub struct Context {
    pub outputs: Vec<Output>,
    pub layout_manager: Option<Main<RiverLayoutManagerV2>>,
}

pub struct Options {
    pub rule: Rule,
    pub main_amount: u32,
    pub main_index: u32,
    pub main_factor: f64,
    pub view_padding: u32,
    pub windows: Vec<Window>,
}

pub struct Output {
    pub default: Tag,
    pub resized: bool,
    pub focused: usize,
    pub output: WlOutput,
    pub dimension: Area,
    pub smart_padding: bool,
    pub view_amount: u32,
    pub outer_padding: u32,
    pub tags: [Option<Tag>; 32],
}

pub struct Tag {
    pub outer: Layout,
    pub options: Options,
    pub inner: Vec<Layout>,
}

#[derive(Clone, Debug)]
pub struct Window {
    pub tags: u32,
    pub app_id: String,
    pub area: Option<Area>,
}

#[derive(Copy, Clone, Debug)]
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

#[derive(Debug)]
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
                rule: Rule::None,
                view_padding: 0,
                main_factor: 0.55,
                main_index: 0,
                main_amount: 1,
                windows: Vec::new(),
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
                outer_padding: 0,
                view_amount: 0,
                resized: false,
                smart_padding: false,
                tags: Default::default(),
                default: {
                    Tag {
                        options: Options::new(),
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
                view_count,
                usable_width,
                usable_height,
                serial: _,
                mut tags,
            } => {
                self.view_amount = view_count;
                if !self.resized {
                    self.dimension = Area::from(0, 0, usable_width, usable_height);
                }
                if !self.smart_padding || view_count > 1 {
                    self.dimension.apply_padding(self.outer_padding);
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
                let list = match self.tags[self.focused].as_mut() {
                    Some(tag) => {
                        tag.options
                            .windows
                            .append(&mut self.default.options.windows);
                        tag.update(serial, &layout, self.view_amount, Rectangle::Area(self.dimension))
                    }
                    None => self
                        .default
                        .update(serial, &layout, self.view_amount, Rectangle::Area(self.dimension))
                };
                for windows in list {
                    let area = windows.area();
                    layout.push_view_dimensions(serial, area.x as i32, area.y as i32, area.w, area.h)
                }
                layout.commit(serial);
            }
            Event::SetIntValue { name, value } => match name.as_ref() {
                "main_amount" | "main_index" | "view_padding" => {
                    if let Some(tag) = self.tags[self.focused].as_mut() {
                        if value >= 0 {
                            match name.as_ref() {
                                "main_amount" => tag.options.main_amount = value as u32,
                                "main_index" => tag.options.main_index = value as u32,
                                "view_padding" => tag.options.view_padding = value as u32,
                                _ => {}
                            }
                        }
                    }
                }
                "outer_padding" => self.outer_padding = value as u32,
                _ => {}
            },
            Event::ModIntValue { name, mut delta } => match name.as_ref() {
                "main_amount" | "main_index" | "view_padding" => {
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
                            "view_padding" => {
                                tag.options.view_padding =
                                    ((tag.options.view_padding as i32) + delta) as u32
                            }
                            _ => {}
                        }
                    }
                }
                "outer_padding" => {
                    self.outer_padding = ((self.outer_padding as i32) + delta) as u32
                }
                "xoffset" => {
                    if delta != 0 {
                        if delta < 0 {
                            self.dimension.x = 0;
                            delta = delta * (-1);
                        } else {
                            self.dimension.x = delta as u32;
                        }
                        self.dimension.w -= delta as u32;
                        self.resized = true;
                    } else {
                        self.resized = false;
                    }
                }
                "yoffset" => {
                    if delta != 0 {
                        if delta < 0 {
                            self.dimension.y = 0;
                            delta = delta * (-1);
                        } else {
                            self.dimension.y = delta as u32;
                        }
                        self.dimension.h -= delta as u32;
                        self.resized = true;
                    } else {
                        self.resized = false;
                    }
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
    pub fn update(
        &mut self,
        serial: u32,
        layout: &Main<RiverLayoutV2>,
        view_amount: u32,
        mut area: Rectangle,
    ) -> Vec<Rectangle> {
        let mut list = Vec::new();
        let slave_amount;
        let frames_available = self.inner.len() as u32;
        let frame_amount = {
            let main = self.options.main_amount > 1
                && frames_available > 1
                && view_amount > self.options.main_amount;
            if main {
                if view_amount - self.options.main_amount < frames_available {
                    1 + view_amount - self.options.main_amount
                } else {
                    frames_available
                }
            } else if self.options.main_amount >= view_amount {
                1
            } else if view_amount > frames_available {
                frames_available
            } else {
                view_amount
            }
        };
        let main_amount = if self.options.main_index + self.options.main_amount <= view_amount
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
        area.generate(
            serial,
            layout,
            &mut self.options,
            frame_amount,
            self.outer,
            &mut list,
            true,
            true,
        );
        let mut reste = if main_amount > 0 {
            zoom(&mut list, self.options.main_index as usize);
            slave_amount = (view_amount - main_amount) / (frame_amount - 1);
            (view_amount - main_amount) % (frame_amount - 1)
        } else {
            slave_amount = view_amount / frame_amount;
            view_amount % frame_amount
        };

        let mut windows = Vec::new();
        for (i, rect) in list.iter_mut().enumerate() {
            let amount = if i == 0 && main_amount != 0 {
                main_amount
            } else {
                if reste > 0 {
                    reste -= 1;
                    slave_amount + 1
                } else {
                    slave_amount
                }
            };
            rect.generate(
                serial,
                layout,
                &mut self.options,
                amount,
                self.inner[i],
                &mut windows,
                false,
                false,
            )
        }
        windows
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
