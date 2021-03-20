use super::layout;
use super::options::Layout;
use super::options::Options;
use crate::wayland::{
    river_layout_unstable_v1::{
        zriver_layout_manager_v1::ZriverLayoutManagerV1, zriver_layout_v1::ZriverLayoutV1,
    },
    river_options_unstable_v1::zriver_options_manager_v1::ZriverOptionsManagerV1,
};
use wayland_client::protocol::wl_output::WlOutput;
use wayland_client::EventQueue;
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
    pub output: Option<WlOutput>,
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
    // pub index: u32,
    // state: State,
}

impl Context {
    pub fn new(namespace: String) -> Context {
        return {
            Context {
                layout_manager: None,
                options: Options::new(),
                running: false,
                options_manager: None,
                namespace: namespace,
                output: None,
                focused: 0,
                tags: Vec::with_capacity(256),
            }
        };
    }
    pub fn init(&mut self) {
        self.options.init(self.clone());
        self.update_focus(self.options.tagmask);
        // self.tags.push(Tag::new(&self));
    }
    pub fn update(&mut self) {
        if !self.options.state {
            self.destroy();
        }
        if self.tags.len() == 0 {
            self.options.debug();
            self.tags.push(Tag::new(&self));
        } else {
            self.options.debug();
            self.tags[0].update(&mut self.options);
        }
    }
    pub fn update_focus(&mut self, tagmask: u32) {
        self.focused = tagmask;
    }
    pub fn destroy(&self) {
        let output = self.output.as_ref().unwrap();
        self.options_manager.clone().unwrap().destroy();
        self.destroy_handle("main-index", output);
        self.destroy_handle("main-count", output);
        self.destroy_handle("main-factor", output);
        self.destroy_handle("view-padding", output);
        self.destroy_handle("outer-padding", output);
    }
    pub fn destroy_handle(&self, name: &str, output: &WlOutput) {
        let option = self
            .options_manager
            .clone()
            .unwrap()
            .get_option_handle(String::from(name), Some(output));
        option.destroy();
    }
}

pub fn tag_index(tagmask: &u32) -> usize {
    usize::from_str_radix(&tagmask.to_string(), 2).unwrap()
}

impl Tag {
    pub fn new(context: &Context) -> Tag {
        return {
            Tag {
                reference: (Layout::Full, 1, State::Frame),
                main: Vec::new(),
                serial: context.options.serial,
                client_count: 0,
                layout: context.options.arguments.clone(),
                windows: Vec::new(),
            }
        };
    }
    pub fn restore(&self, options: &Options) {
        for frame in &self.windows {
            options.push_dimensions(&frame);
        }
        options.commit();
    }
    pub fn is_set(&self) -> bool {
        if self.client_count > 0 {
            return true;
        }
        false
    }
    pub fn update(&mut self, options: &mut Options) {
        if options.view_amount <= 1 {
            options.outer_padding = 0;
            options.view_padding = 0;
        }

        let layout = options.parse(self);
        layout::new(Frame::new(options), options, self.reference, &mut self.main);

        let mut i = 0;
        for frame in &self.main {
            layout::new(frame.clone(), options, layout[i], &mut self.windows);
            i += 1;
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
}
