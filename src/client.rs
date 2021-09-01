use super::layout::Layout;
use super::lexer;
use crate::wayland::{
    river_layout_v3::river_layout_manager_v3::RiverLayoutManagerV3,
    river_layout_v3::river_layout_v3::Event,
};
use wayland_client::protocol::wl_output::WlOutput;
use wayland_client::Main;

// Holds all the globals necessary to operate the client
pub struct Globals {
    pub namespace: String,
    pub layout_manager: Option<Main<RiverLayoutManagerV3>>,
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
    pub name: String,
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

impl Globals {
    pub fn new(namespace: String) -> Globals {
        {
            Globals {
                namespace,
                layout_manager: None,
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
        layout_manager: Option<&Main<RiverLayoutManagerV3>>,
        namespace: String,
    ) {
        // A generic default configuration used when a Tag isn't defined
        let default: Tag = {
            Tag {
                name: String::from("kile"),
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
        let layout = layout_manager
            .expect("Compositor doesn't implement river_layout_v3")
            .get_layout(&self.output, namespace.clone());
        let mut view_padding = 0;
        let mut outer_padding = 0;
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
                let layout_name = if self.reload {
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
                            tag.update(&mut windows, view_count, self.dimension);
                            tag.name.as_str()
                        }
                        None => {
                            default.update(&mut windows, view_count, self.dimension);
                            default.name.as_str()
                        }
                    }
                } else {
                    "no reload"
                };
                self.reload = true;
                for area in &mut windows {
                    if !self.smart_padding || view_count > 1 {
                        area.apply_padding(view_padding);
                    }
                    layout.push_view_dimensions(
                        area.x as i32,
                        area.y as i32,
                        area.w,
                        area.h,
                        serial,
                    )
                }
                layout.commit(layout_name.to_owned(), serial);
            }
            Event::NamespaceInUse => {
                println!("Namespace already in use.");
            }
            // All String events are delegated to the lexer
            Event::UserCommand { command } => {
                let (command, value) = lexer::format(command.as_str());
                match command {
                    "outer_padding" => {
                        if let Ok(value) = value.parse::<i32>() {
                            outer_padding = value;
                        }
                    }
                    "view_padding" => {
                        if let Ok(value) = value.parse::<i32>() {
                            view_padding = value - view_padding;
                            self.view_padding = value;
                            if windows.len() > 0 {
                                self.reload = false;
                            }
                        }
                    }
                    "mod_outer_padding" => {
                        if let Ok(delta) = value.parse::<i32>() {
                            outer_padding += delta;
                        }
                    }
                    "mod_view_padding" => {
                        if let Ok(delta) = value.parse::<i32>() {
                            if (self.view_padding as i32) + delta >= 0 {
                                self.view_padding += delta;
                                view_padding = delta;
                                if windows.len() > 0 {
                                    self.reload = false;
                                }
                            }
                        }
                    }
                    "main_factor" => {
                        if let Some(tag) = self.tags[self.focused].as_mut() {
                            if let Ok(value) = value.parse::<f64>() {
                                if value > 0.0 && value < 1.0 {
                                    tag.parameters.main_factor = value
                                }
                            }
                        }
                    }
                    "mod_main_factor" => {
                        if let Some(tag) = self.tags[self.focused].as_mut() {
                            if let Ok(delta) = value.parse::<f64>() {
                                if delta <= tag.parameters.main_factor {
                                    tag.parameters.main_factor += delta;
                                }
                            }
                        }
                    }
                    "main_amount" => {
                        if let Some(tag) = self.tags[self.focused].as_mut() {
                            if let Ok(value) = value.parse::<u32>() {
                                tag.parameters.main_amount = value
                            }
                        }
                    }
                    "mod_main_amount" => {
                        if let Some(tag) = self.tags[self.focused].as_mut() {
                            if let Ok(delta) = value.parse::<i32>() {
                                if (tag.parameters.main_amount as i32) + delta >= 0 {
                                    tag.parameters.main_amount =
                                        ((tag.parameters.main_amount as i32) + delta) as u32
                                }
                            }
                        }
                    }
                    "main_index" => {
                        if let Some(tag) = self.tags[self.focused].as_mut() {
                            if let Ok(value) = value.parse::<u32>() {
                                tag.parameters.main_index = value;
                            }
                        }
                    }
                    "mod_main_index" => {
                        if let Some(tag) = self.tags[self.focused].as_mut() {
                            if let Ok(delta) = value.parse::<i32>() {
                                if (tag.parameters.main_index as i32) + delta >= 0 {
                                    tag.parameters.main_index =
                                        ((tag.parameters.main_index as i32) + delta) as u32
                                }
                            }
                        }
                    }
                    "xoffset" => {
                        if let Ok(delta) = value.parse::<i32>() {
                            if delta < 0 {
                                self.dimension.x = 0;
                            } else {
                                self.dimension.x = delta.abs() as u32;
                            }
                            self.dimension.w -= delta.abs() as u32;
                            self.resize = true;
                        }
                    }
                    "yoffset" => {
                        if let Ok(delta) = value.parse::<i32>() {
                            if delta < 0 {
                                self.dimension.y = 0;
                            } else {
                                self.dimension.y = delta.abs() as u32;
                            }
                            self.dimension.h -= delta.abs() as u32;
                            self.resize = true;
                        }
                    }
                    "dimension" => {
                        let mut fields = value.split_whitespace();
                        self.dimension = {
                            self.resize = true;
                            Area {
                                x: fields.next().unwrap_or("0").parse::<u32>().unwrap(),
                                y: fields.next().unwrap_or("0").parse::<u32>().unwrap(),
                                w: fields.next().unwrap_or("500").parse::<u32>().unwrap(),
                                h: fields.next().unwrap_or("500").parse::<u32>().unwrap(),
                            }
                        }
                    }
                    "resize" => {
                        self.resize = if let Ok(ans) = value.parse::<bool>() {
                            ans
                        } else {
                            false
                        }
                    }
                    "smart_padding" => {
                        if let Ok(ans) = value.parse::<bool>() {
                            self.smart_padding = ans;
                        }
                    }
                    "clear" => {
                        match value {
                            "all" => self.tags = Default::default(),
                            "focused" => self.tags[self.focused] = None,
                            _ => match value.parse::<usize>() {
                                Ok(int) => {
                                    if int > 0 && int < 33 {
                                        self.tags[int - 1] = None
                                    }
                                }
                                Err(_) => {}
                            },
                        }
                    }
                    _ => lexer::main(&mut self, command, value),
                }
            }
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
