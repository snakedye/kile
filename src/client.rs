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
    pub amount: u32,
    pub index: u32,
    pub ratio: f64,
}

#[derive(Copy, Clone)]
pub enum Order {
    Ascend,
    Descend,
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
    // Order the tags are sorted
    pub order: Order,
    // Dimensions of the layout area
    pub dimension: Area,
    pub view_padding: i32,
    pub outer_padding: i32,
    pub smart_padding: bool,
    // The configuration of all Tags
    pub tags: [Option<Tag>; 32],
}

// The configuration of a Tag
pub struct Tag {
    pub name: String,
    pub layout: Layout,
    pub parameters: Parameters,
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
                outer_padding: 0,
                smart_padding: false,
                order: Order::Ascend,
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
        let mut default: Tag = {
            Tag {
                name: "kile".to_owned(),
                parameters: {
                    Parameters {
                        amount: 1,
                        index: 0,
                        ratio: 0.55,
                    }
                },
                layout: Layout::Full,
            }
        };
        let layout = layout_manager
            .expect("Compositor doesn't implement river_layout_v3")
            .get_layout(&self.output, namespace.clone());
        let mut view_padding = 0;
        // A vector holding the geometry of all the views from the most recent layout demand
        let mut views: Vec<Area> = Vec::new();
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
                            self.dimension.apply_padding(self.outer_padding);
                        }
                    }
                    self.focused = tag(tags, &self.order) as usize;
                    match self.tags[self.focused].as_ref() {
                        Some(tag) => {
                            view_padding = self.view_padding;
                            tag.update(&mut views, view_count, self.dimension);
                            tag.name.as_str()
                        }
                        None => {
                            default.update(&mut views, view_count, self.dimension);
                            default.name.as_str()
                        }
                    }
                } else {
                    "reload"
                };
                self.reload = true;
                for area in &mut views {
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
                            self.outer_padding = value;
                        }
                    }
                    "view_padding" => {
                        if let Ok(value) = value.parse::<i32>() {
                            view_padding = value - view_padding;
                            self.view_padding = value;
                            if !views.is_empty() {
                                self.reload = false;
                            }
                        }
                    }
                    "mod_outer_padding" => {
                        if let Ok(delta) = value.parse::<i32>() {
                            self.outer_padding += delta;
                        }
                    }
                    "mod_view_padding" => {
                        if let Ok(delta) = value.parse::<i32>() {
                            if (self.view_padding as i32) + delta >= 0 {
                                self.view_padding += delta;
                                view_padding = delta;
                                if !views.is_empty() {
                                    self.reload = false;
                                }
                            }
                        }
                    }
                    "main_ratio" => {
                        if let Some(tag) = self.tags[self.focused].as_mut() {
                            if let Ok(value) = value.parse::<f64>() {
                                tag.parameters.ratio = value.clamp(0.0, 1.0);
                            }
                        }
                    }
                    "mod_main_ratio" => {
                        if let Some(tag) = self.tags[self.focused].as_mut() {
                            if let Ok(delta) = value.parse::<f64>() {
                                if delta <= tag.parameters.ratio {
                                    tag.parameters.ratio += delta;
                                }
                            }
                        }
                    }
                    "main_amount" => {
                        if let Some(tag) = self.tags[self.focused].as_mut() {
                            if let Ok(value) = value.parse::<u32>() {
                                tag.parameters.amount = value
                            }
                        }
                    }
                    "mod_main_amount" => {
                        if let Some(tag) = self.tags[self.focused].as_mut() {
                            if let Ok(delta) = value.parse::<i32>() {
                                if (tag.parameters.amount as i32) + delta >= 0 {
                                    tag.parameters.amount =
                                        ((tag.parameters.amount as i32) + delta) as u32
                                }
                            }
                        }
                    }
                    "main_index" => {
                        if let Some(tag) = self.tags[self.focused].as_mut() {
                            if let Ok(value) = value.parse::<u32>() {
                                tag.parameters.index = value;
                            }
                        }
                    }
                    "mod_main_index" => {
                        if let Some(tag) = self.tags[self.focused].as_mut() {
                            if let Ok(delta) = value.parse::<i32>() {
                                if (tag.parameters.index as i32) + delta >= 0 {
                                    tag.parameters.index =
                                        ((tag.parameters.index as i32) + delta) as u32
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
                                x: fields
                                    .next()
                                    .unwrap_or_default()
                                    .parse::<u32>()
                                    .unwrap_or(self.dimension.x),
                                y: fields
                                    .next()
                                    .unwrap_or_default()
                                    .parse::<u32>()
                                    .unwrap_or(self.dimension.y),
                                w: fields
                                    .next()
                                    .unwrap_or_default()
                                    .parse::<u32>()
                                    .unwrap_or(self.dimension.w),
                                h: fields
                                    .next()
                                    .unwrap_or_default()
                                    .parse::<u32>()
                                    .unwrap_or(self.dimension.h),
                            }
                        }
                    }
                    "resize" => {
                        if let Ok(ans) = value.parse::<bool>() {
                            self.resize = ans;
                        }
                    }
                    "smart_padding" => {
                        if let Ok(ans) = value.parse::<bool>() {
                            self.smart_padding = ans;
                        }
                    }
                    "order" => match value {
                        "ascend" => self.order = Order::Ascend,
                        "descend" => self.order = Order::Descend,
                        _ => {}
                    },
                    "default" => {
                        let (name, layout) = if let Some(data) = value.split_once('\n') {
                            data
                        } else {
                            lexer::Expression::new(value).split_ounce(' ').drop()
                        };
                        default.name = name.to_owned();
                        default.layout = lexer::layout(layout);
                    }
                    "clear" => match value {
                        "all" => self.tags = Default::default(),
                        "default" => default.layout = Layout::Full,
                        "focused" => self.tags[self.focused] = None,
                        _ => match value.parse::<usize>() {
                            Ok(int) => {
                                if int > 0 && int < 33 {
                                    self.tags[int - 1] = None
                                }
                            }
                            Err(_) => {}
                        },
                    },
                    _ => lexer::main(&mut self, command, value),
                }
            }
        });
    }
}

impl Tag {
    fn update(&self, views: &mut Vec<Area>, view_amount: u32, area: Area) {
        views.clear();
        area.generate(views, &self.layout, &self.parameters, view_amount, true);
    }
}

fn tag(tagmask: u32, order: &Order) -> u32 {
    let mut int = 0;
    let mut current: u32;
    while {
        current = 1 << int;
        current < tagmask
    } {
        if current != tagmask && (tagmask / current) % 2 != 0 {
            if let Order::Descend = order {
                int = tag(tagmask - current, order);
            }
            break;
        }
        int += 1;
    }
    int
}
