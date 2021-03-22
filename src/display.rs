use super::options::{Layout, Options};
use crate::wayland::{
    river_layout_unstable_v1::zriver_layout_manager_v1::ZriverLayoutManagerV1,
    river_options_unstable_v1::zriver_options_manager_v1::ZriverOptionsManagerV1,
    river_status_unstable_v1::zriver_status_manager_v1::ZriverStatusManagerV1,
};
use wayland_client::protocol::{
    wl_output::WlOutput,
    wl_seat::WlSeat,
};
use wayland_client::Main;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum State {
    Frame,
    Window,
}

#[derive(Clone, Debug)]
pub struct Context {
    pub namespace: String, // Namespace is either manual or dynamic
    pub running: bool,
    pub options: Options,
    pub layout_manager: Option<Main<ZriverLayoutManagerV1>>,
    pub options_manager: Option<Main<ZriverOptionsManagerV1>>,
    pub status_manager: Option<Main<ZriverStatusManagerV1>>,
    pub output: Option<WlOutput>,
    pub seat: Option<WlSeat>,
    pub tags: Vec<Tag>, // It's the amount of possible tags, it might be too much to set all possible tags
    pub focused: u32,
}

#[derive(Clone, Debug)]
pub struct Tag {
    pub serial: u32,
    pub main: Vec<Frame>,
    pub reference: (Layout, u32, State),
    pub client_count: u32,
    pub windows: Vec<Frame>,
    pub layout: String,
}

#[derive(Copy, Clone, Debug)]
pub struct Frame {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

impl Context {
    pub fn new(namespace: String) -> Context {
        return {
            Context {
                layout_manager: None,
                options: Options::new(),
                running: false,
                options_manager: None,
                status_manager: None,
                namespace: namespace,
                output: None,
                seat: None,
                focused: 0,
                tags: Vec::with_capacity(33),
            }
        };
    }
    pub fn init(&mut self) {
        self.options.init(self.clone());
        for _i in 0..33 {
            self.tags.push(Tag::new());
        }
        self.update_focus(self.options.tagmask);
    }
    pub fn update(&mut self) {
        if self.options.state {
            self.focused=tag_index(self.options.tagmask);
            self.tags[self.focused as usize].update(&mut self.options);
        } else {
            self.destroy();
            self.running=false;
        }
    }
    pub fn update_focus(&mut self, tagmask: u32) {
        self.focused = tagmask;
    }
    pub fn destroy(&mut self) {
        self.destroy_handle("main_index");
        self.destroy_handle("main_count");
        self.destroy_handle("main_factor");
        self.destroy_handle("view_padding");
        self.destroy_handle("outer_padding");
        self.destroy_handle("kile");
        self.status_manager.as_ref().unwrap().destroy();
        self.layout_manager.as_ref().unwrap().destroy();
        self.options_manager.as_ref().unwrap().destroy();
    }
    pub fn destroy_handle(&mut self, name: &str) {
        let option = self
            .options_manager
            .clone()
            .unwrap()
            .get_option_handle(String::from(name), Some(&self.output.as_ref().unwrap()));
        option.destroy();
    }
}

pub fn tag_index(mut tagmask: u32) -> u32 {
    // tagmask+=1;
    let mut tag=0;
    while tagmask/2 >= 1 {
        tagmask/=2;
        tag+=1;
    };
    if tag > 32 {
        33
    } else { tag }
}

impl Tag {
    pub fn new() -> Tag {
        return {
            Tag {
                reference: (Layout::Full, 1, State::Frame),
                main: Vec::new(),
                serial: 0,
                client_count: 0,
                windows: Vec::new(),
                layout: String::new(),
            }
        };
    }
    pub fn init(&mut self, options: &mut Options) {
        self.reference = (Layout::Full, 1, State::Frame);
        self.main = Vec::new();
        self.serial = options.serial;
        self.client_count = options.view_amount;

        match tag_index(options.tagmask)+1 {
            1 => self.layout=String::from("vtt"),
            2 => self.layout=String::from("vtth"),
            _ => self.layout=options.layout.clone(),
        }

        self.windows = Vec::new();
    }
    pub fn restore(&self, options: &Options) {
        for frame in &self.windows {
            options.push_dimensions(&frame);
        }
        options.commit();
    }
    pub fn update(&mut self, options: &mut Options) {
        self.init(options);

        if options.view_amount > 0 {
            let layout = options.parse_layout(self);
            Frame::new(options).new_layout(options, self.reference, &mut self.main);

            let mut i = 0;
            for frame in &self.main {
                if layout[i].1 > 0 {
                    frame
                        .clone()
                        .new_layout(options, layout[i], &mut self.windows);
                }
                i += 1;
            }
        }
        self.restore(options);
        self.client_count = options.view_amount;
        self.clean();
    }
    pub fn clean(&mut self) {
        self.reference = (Layout::Full, 1, State::Frame);
        self.main = Vec::new();
        self.windows = Vec::new();
    }
}

impl Frame {
    pub fn new(options: &Options) -> Frame {
        let mut frame = {
            Frame {
                x: options.view_padding,
                y: options.view_padding,
                w: options.usable_width - options.view_padding,
                h: options.usable_height - options.view_padding,
            }
        };
        frame.apply_padding(options.outer_padding);
        frame
    }
    pub fn apply_padding(&mut self, padding: u32) {
        if 2 * padding < self.h && 2 * padding < self.w {
            self.x += padding;
            self.y += padding;
            self.w -= 2 * padding;
            self.h -= 2 * padding;
        }
    }
    pub fn new_layout(
        &mut self,
        options: &Options,
        layout: (Layout, u32, State),
        frames: &mut Vec<Frame>,
    ) {
        let (layout, client_count, state) = layout;
        let mut is_main = 0;

        if client_count > 0 {
            match layout {
                Layout::Tab => {
                    // Add eww titlebar eventually
                    self.h -= options.view_padding;
                    for _i in 0..client_count {
                        frames.push(self.clone());
                        self.h -= 30;
                        self.y += 30;
                    }
                }
                Layout::Horizontal => {
                    let mut slave_area = self.clone();
                    let mut main_area = self.clone();
                    let reste = self.h % client_count;
                    if state == State::Frame {
                        main_area.h = if options.main_count > 0
                            && options.main_count < options.view_amount
                            && options.main_index < options.view_amount
                        {
                            is_main = 1;
                            (self.h * (options.main_factor * 100.0) as u32) / (50 * client_count)
                        } else {
                            0
                        };
                        slave_area.h -= main_area.h;
                    }
                    for i in 0..client_count {
                        if state == State::Frame
                            && i == options.main_index
                            && options.main_count < options.view_amount
                            && options.main_index < options.view_amount
                        {
                            self.h = main_area.h;
                        } else {
                            self.h = slave_area.h / client_count;
                        }
                        self.h -= options.view_padding;
                        if i == 0 {
                            self.h += reste;
                        }

                        frames.push(*self);
                        self.y += self.h + options.view_padding;
                    }
                }
                Layout::Vertical => {
                    let mut slave_area = self.clone();
                    let mut main_area = self.clone();
                    let reste = self.w % client_count;
                    if state == State::Frame {
                        main_area.w = if options.main_count > 0
                            && options.main_count < options.view_amount
                            && options.main_index < options.view_amount
                        {
                            is_main = 1;
                            (self.w * (options.main_factor * 100.0) as u32) / (50 * client_count)
                        } else {
                            0
                        };
                        slave_area.w -= main_area.w;
                    }
                    for i in 0..client_count {
                        if state == State::Frame
                            && i == options.main_index
                            && options.main_count < options.view_amount
                            && options.main_index < options.view_amount
                        {
                            self.w = main_area.w;
                        } else {
                            self.w = slave_area.w / (client_count - is_main);
                        }
                        self.w -= options.view_padding;
                        if i == 0 {
                            self.w += reste;
                        }

                        frames.push(*self);
                        self.x += self.w + options.view_padding;
                    }
                }
                Layout::Full => {
                    for _i in 0..client_count {
                        self.y -= options.view_padding;
                        frames.push(*self);
                    }
                }
            }
        }
    }
}
