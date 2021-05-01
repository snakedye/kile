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
    pub outer_padding: u32,
    pub tags: [Option<Tag>; 32],
}

pub struct Tag {
    pub options: Options,
    pub layout: Layout,
}

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
                resized: false,
                smart_padding: false,
                tags: Default::default(),
                default: {
                    Tag {
                        options: Options::new(),
                        layout: Layout::Full,
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
                        tag.update(
                            serial,
                            &layout,
                            Rectangle::Area(self.dimension),
                        )
                    }
                    None => self.default.update(
                        serial,
                        &layout,
                        Rectangle::Area(self.dimension),
                    ),
                };
                for windows in list {
                    let area = windows.area();
                    layout.push_view_dimensions(
                        serial,
                        area.x as i32,
                        area.y as i32,
                        area.w,
                        area.h,
                    )
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
    pub fn update(
        &mut self,
        serial: u32,
        layout: &Main<RiverLayoutV2>,
        mut area: Rectangle,
    ) -> Vec<Rectangle> {
        let mut list = Vec::new();
        let view_amount = self.options.windows.len() as u32;
        let parent = match &self.layout {
            Layout::Recursive{ outer:_, inner:_ } => true,
            _ => false
        };
        area.generate(
            serial,
            layout,
            &mut self.options,
            view_amount,
            &self.layout,
            &mut list,
            parent,
            true,
        );
        list
    }
}
