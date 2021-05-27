use super::layout::Layout;
use super::parser;
use crate::wayland::{
    river_layout_v2::river_layout_manager_v2::RiverLayoutManagerV2,
    river_layout_v2::river_layout_v2::Event,
};
use wayland_client::protocol::wl_output::WlOutput;
use wayland_client::Main;

pub struct Globals {
    pub outputs: Vec<Output>,
    pub layout_manager: Option<Main<RiverLayoutManagerV2>>,
}

#[derive(Copy, Clone, Debug)]
pub struct Parameters {
    pub main_amount: u32,
    pub main_index: u32,
    pub main_factor: f64,
    pub view_padding: i32,
}

pub struct Output {
    pub output: WlOutput,
    pub default: Tag,
    pub focused: usize,
    pub reload: bool,
    pub resize: bool,
    pub dimension: Area,
    pub outer_padding: i32,
    pub smart_padding: bool,
    pub tags: [Option<Tag>; 32],
}

pub struct Tag {
    pub rule: Rule,
    pub parameters: Parameters,
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

impl Globals {
    pub fn new() -> Globals {
        return {
            Globals {
                layout_manager: None,
                outputs: Vec::new(),
            }
        };
    }
}

impl Parameters {
    pub fn new() -> Parameters {
        return {
            Parameters {
                view_padding: 5,
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
                reload: true,
                resize: false,
                outer_padding: 0,
                smart_padding: false,
                tags: Default::default(),
                default: {
                    Tag {
                        rule: Rule::None,
                        parameters: Parameters::new(),
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
        let mut view_padding = 0;
        let mut windows: Vec<Area> = Vec::new();
        layout.quick_assign(move |layout, event, _| match event {
            Event::LayoutDemand {
                view_count,
                usable_width,
                usable_height,
                serial,
                tags,
            } => {
                if self.reload {
                    if !self.resize {
                        self.dimension = {
                            Area {
                                x: 0,
                                y: 0,
                                w: usable_width,
                                h: usable_height,
                            }
                        };
                        if !self.smart_padding || view_count > 1 {
                            self.dimension.apply_padding(self.outer_padding);
                        }
                    }
                    self.focused = {
                        let mut i = 0;
                        while (1 << i) <= tags {
                            i += 1;
                            if i == 32 { break }
                        }
                        i as usize
                    };
                    match self.tags[self.focused].as_mut() {
                        Some(tag) => {
                            view_padding = tag.parameters.view_padding;
                            tag.update(&mut windows, view_count, self.dimension)
                        }
                        None => self
                            .default
                            .update(&mut windows, view_count, self.dimension),
                    };
                }
                self.reload = true;
                for area in &mut windows {
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
                "outer_padding" => self.outer_padding = value,
                _ => {
                    if let Some(tag) = self.tags[self.focused].as_mut() {
                        if value >= 0 {
                            match name.as_ref() {
                                "main_amount" => tag.parameters.main_amount = value as u32,
                                "main_index" => tag.parameters.main_index = value as u32,
                                "view_padding" => {
                                    let delta = value - tag.parameters.view_padding;
                                    tag.parameters.view_padding = value;
                                    view_padding = delta;
                                    self.reload = false;
                                }
                                _ => {}
                            }
                        }
                    }
                }
            },
            Event::ModIntValue { name, delta } => match name.as_ref() {
                "outer_padding" => {
                    if self.outer_padding as i32 >= delta {
                        self.outer_padding += delta
                    }
                }
                "xoffset" => {
                    if delta < 0 {
                        self.dimension.x = 0;
                    } else {
                        self.dimension.x = delta.abs() as u32;
                    }
                    self.dimension.w -= delta.abs() as u32;
                    self.resize = true;
                }
                "yoffset" => {
                    if delta < 0 {
                        self.dimension.y = 0;
                    } else {
                        self.dimension.y = delta.abs() as u32;
                    }
                    self.dimension.h -= delta.abs() as u32;
                    self.resize = true;
                }
                _ => {
                    if let Some(tag) = self.tags[self.focused].as_mut() {
                        match name.as_ref() {
                            "main_amount" => {
                                tag.parameters.main_amount =
                                    ((tag.parameters.main_amount as i32) + delta) as u32
                            }
                            "main_index" => {
                                tag.parameters.main_index =
                                    ((tag.parameters.main_index as i32) + delta) as u32
                            }
                            "view_padding" => {
                                if tag.parameters.view_padding + delta >= 0 {
                                    tag.parameters.view_padding += delta;
                                    view_padding = delta;
                                    self.reload = false;
                                }
                            }
                            _ => {}
                        }
                    }
                }
            },
            Event::SetFixedValue { name, value } => {
                if name == "main_factor" {
                    if let Some(tag) = self.tags[self.focused].as_mut() {
                        if value > 0.0 && value < 1.0 {
                            tag.parameters.main_factor = value
                        }
                    }
                }
            }
            Event::ModFixedValue { name, delta } => {
                if name == "main_factor" {
                    if let Some(tag) = self.tags[self.focused].as_mut() {
                        if delta <= tag.parameters.main_factor {
                            tag.parameters.main_factor += delta;
                        }
                    }
                }
            }
            Event::SetStringValue { name, value } => parser::main(&mut self, name, value),
        });
    }
}

impl Tag {
    fn update(&mut self, list: &mut Vec<Area>, view_amount: u32, area: Area) {
        let parent;
        *list = Vec::new();
        match &self.layout {
            Layout::Recursive { outer: _, inner: _ } => {
                parent = true;
            }
            _ => parent = false,
        };
        area.generate(&self.parameters, view_amount, &self.layout, list, parent, true);
    }
}
