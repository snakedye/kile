use super::layout::Layout;
use super::lexer;
use crate::wayland::{
    river_layout_v2::river_layout_manager_v2::RiverLayoutManagerV2,
    river_layout_v2::river_layout_v2::Event,
};
use wayland_client::protocol::wl_output::WlOutput;
use wayland_client::Main;

// Holds all the globals necessary to operate the client
pub struct Globals {
    pub outputs: Vec<Output>,
    pub layout_manager: Option<Main<RiverLayoutManagerV2>>,
}

// Parameters necessary to generate a layout
#[derive(Copy, Clone)]
pub struct Parameters {
    pub main_amount: u32,
    pub main_index: u32,
    pub main_factor: f64,
}

// The state of an Output
pub struct Output {
    pub output: Main<WlOutput>,
    // This is the index of the focused Tag
    pub focused: usize,
    // Defines if a layout should regenerated or not
    pub reload: bool,
    // Defines if a the layout area should reajusted to the output dimension or not
    pub resize: bool,
    // Dimensions of the output
    pub dimension: Area,
    pub view_padding: i32,
    pub smart_padding: bool,
    // The configuration of all Tags
    pub tags: [Option<Tag>; 32],
}

// The configuration of a Tag
pub struct Tag {
    pub parameters: Parameters,
    pub layout: Layout,
}

#[derive(Copy, Clone)]
pub struct Area {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

// A generic default configuration used when a Tag isn't defined
static DEFAULT: Tag = {
    Tag {
        parameters: {
            Parameters {
                main_amount: 1,
                main_index: 0,
                main_factor: 0.55,
            }
        },
        layout: Layout::Full,
    }
};

impl Globals {
    pub fn new() -> Globals {
        {
            Globals {
                layout_manager: None,
                outputs: Vec::new(),
            }
        }
    }
}

impl Output {
    pub fn new(output: Main<WlOutput>) -> Output {
        {
            Output {
                output,
                dimension: Area {
                    x: 0,
                    y: 0,
                    w: 0,
                    h: 0,
                },
                focused: 0,
                reload: true,
                resize: false,
                view_padding: 0,
                smart_padding: false,
                tags: Default::default(),
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
        let mut outer_padding = 0;
        let mut view_padding = 0;
        // A vector holding the geometry of all the windows from the most recent layout demand
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
                            self.dimension.apply_padding(outer_padding);
                        }
                    }
                    self.focused = tag(tags) as usize;
                    match self.tags[self.focused].as_ref() {
                        Some(tag) => {
                            view_padding = self.view_padding;
                            println!("windows count: {}", view_count);
                            println!("{:#?}", tag.layout);
                            tag.update(&mut windows, view_count, self.dimension)
                        }
                        None => DEFAULT.update(&mut windows, view_count, self.dimension),
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
                tags: _,
                app_id: _,
                serial: _,
            } => {}
            Event::NamespaceInUse => {
                println!("Namespace already in use.");
            }
            Event::AdvertiseDone { serial: _ } => {}
            Event::SetIntValue { name, value } => match name.as_ref() {
                "outer_padding" => outer_padding = value,
                "view_padding" => {
                    view_padding = value - view_padding;
                    self.view_padding = value;
                    if windows.len() > 0 {
                        self.reload = false;
                    }
                }
                _ => {
                    if let Some(tag) = self.tags[self.focused].as_mut() {
                        if value >= 0 {
                            match name.as_ref() {
                                "main_amount" => tag.parameters.main_amount = value as u32,
                                "main_index" => tag.parameters.main_index = value as u32,
                                _ => {}
                            }
                        }
                    }
                }
            },
            Event::ModIntValue { name, delta } => match name.as_ref() {
                "outer_padding" => {
                    if outer_padding as i32 >= delta {
                        outer_padding += delta;
                    }
                }
                "view_padding" => {
                    if (self.view_padding as i32) + delta >= 0 {
                        self.view_padding += delta;
                        view_padding = delta;
                        if windows.len() > 0 {
                            self.reload = false;
                        }
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
                                if (tag.parameters.main_amount as i32) + delta >= 0 {
                                    tag.parameters.main_amount =
                                        ((tag.parameters.main_amount as i32) + delta) as u32
                                }
                            }
                            "main_index" => {
                                if (tag.parameters.main_index as i32) + delta >= 0 {
                                    tag.parameters.main_index =
                                        ((tag.parameters.main_index as i32) + delta) as u32
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
            // All String events are delegated to the lexer
            Event::SetStringValue { name, value } => lexer::main(&mut self, name, value),
        });
    }
}

impl Tag {
    fn update(&self, list: &mut Vec<Area>, view_amount: u32, area: Area) {
        *list = Vec::new();
        let parent = match &self.layout {
            Layout::Recursive { outer: _, inner: _ } => true,
            _ => false,
        };
        area.generate(
            &self.parameters,
            view_amount,
            &self.layout,
            list,
            parent,
            true,
        );
    }
}

fn tag(tagmask: u32) -> u32 {
    let mut int = 0;
    let mut current: u32;
    while {
        current = 1 << int;
        current < tagmask
    } {
        int += 1;
        if current != tagmask && (tagmask / current) % 2 != 0 {
            int = tag(tagmask - current);
            break;
        }
    }
    int
}
