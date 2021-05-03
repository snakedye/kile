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

#[derive(Copy, Clone, Debug)]
pub struct Options {
    pub main_amount: u32,
    pub main_index: u32,
    pub main_factor: f64,
    pub view_padding: u32,
}

pub struct Output {
    pub output: WlOutput,
    pub default: Tag,
    pub focused: usize,
    pub view_amount: u32,
    pub dimension: Area,
    pub outer_padding: u32,
    pub smart_padding: bool,
    pub tags: [Option<Tag>; 32],
}

pub struct Tag {
    pub rule: Rule,
    pub options: Options,
    pub layout: Layout,
}

#[derive(Copy, Clone, Debug)]
pub struct Area {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

#[derive(Clone, Debug, std::cmp::PartialEq)]
pub enum Rule {
    AppId(String),
    Tag(u32),
    None,
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
                view_padding: 0,
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
                view_amount: 0,
                outer_padding: 0,
                smart_padding: false,
                tags: Default::default(),
                default: {
                    Tag {
                        rule: Rule::None,
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
        let mut resized = false;
        layout.quick_assign(move |layout, event, _| match event {
            Event::LayoutDemand {
                view_count,
                usable_width,
                usable_height,
                serial,
                mut tags,
            } => {
                if !resized {
                    self.dimension = Area::new(0, 0, usable_width, usable_height);
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
                let mut view_padding = 0;
                let windows = match self.tags[self.focused].as_mut() {
                    Some(tag) => {
                        view_padding = tag.options.view_padding;
                        tag.update(view_count, self.dimension)
                    }
                    None => self
                        .default
                        .update(view_count, self.dimension),
                };
                for mut area in windows {
                    if !self.smart_padding || view_count > 1 {
                        area.apply_padding(view_padding);
                    }
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
            Event::AdvertiseView {
                tags,
                app_id,
                serial,
            } => {}
            Event::NamespaceInUse => {
                println!("Namespace already in use.");
            }
            Event::AdvertiseDone { serial } => {}
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
                            "view_padding" => if tag.options.view_padding as i32 >= delta {
                                tag.options.view_padding =
                                    ((tag.options.view_padding as i32) + delta) as u32
                            }
                            _ => {}
                        }
                    }
                }
                "outer_padding" => if self.outer_padding as i32 >= delta {
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
                        resized = true;
                    } else {
                        resized = false;
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
                        resized = true;
                    } else {
                        resized = false;
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

impl Tag {
    pub fn update(&mut self, view_amount: u32, area: Area) -> Vec<Area> {
        let parent;
        let mut list = Vec::new();
        match &self.layout {
            Layout::Recursive { outer: _, inner: _ } => {
                parent = true;
            }
            _ => parent = false,
        };
        area.generate(
            self.options,
            view_amount,
            &self.layout,
            &mut list,
            parent,
            true,
        );
        list
    }
}
