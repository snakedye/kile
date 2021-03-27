use super::options::{Layout, Options};
use crate::wayland::{
    river_layout_unstable_v1::zriver_layout_manager_v1::ZriverLayoutManagerV1,
    river_options_unstable_v1::zriver_options_manager_v1::ZriverOptionsManagerV1,
    river_status_unstable_v1::zriver_status_manager_v1::ZriverStatusManagerV1,
};
use wayland_client::protocol::{wl_output::WlOutput, wl_seat::WlSeat};
use wayland_client::Main;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum State {
    Frame,
    Window,
}

#[derive(Clone, Debug)]
pub struct Context {
    pub namespace: String,
    pub running: bool,
    pub options: Options,
    pub layout_manager: Option<Main<ZriverLayoutManagerV1>>,
    pub options_manager: Option<Main<ZriverOptionsManagerV1>>,
    pub status_manager: Option<Main<ZriverStatusManagerV1>>,
    pub output: Option<WlOutput>,
    pub seat: Option<WlSeat>,
    pub tags: [Option<Tag>; 32], // It's the amount of possible tags, it might be too much to set all possible tags
    pub focused: u32,
}

#[derive(Clone, Debug)]
pub struct Tag {
    pub output: Vec<Frame>,
    pub windows: Vec<Frame>,
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
                running: false,
                focused: 0,
                namespace: namespace,
                options: Options::new(),
                layout_manager: None,
                options_manager: None,
                status_manager: None,
                output: None,
                seat: None,
                tags: Default::default(),
            }
        };
    }
    pub fn init(&mut self) {
        self.options.update(self.clone());
        self.update_focus(self.options.tagmask);
    }
    pub fn update(&mut self) {
        if !self.running {
            self.destroy();
        }
        self.focused = self.options.tagmask;
        match self.tags[self.focused as usize].as_mut() {
            Some(tag) => tag.update(&mut self.options),
            None => {
                let mut tag = Tag::new();
                tag.update(&mut self.options);
                self.tags[self.focused as usize] = Some(tag);
            }
        };
    }
    pub fn update_focus(&mut self, tagmask: u32) {
        self.focused = tagmask;
    }
    pub fn destroy(&mut self) {
        self.destroy_handle("main_index");
        self.destroy_handle("main_amount");
        self.destroy_handle("main_factor");
        self.destroy_handle("view_padding");
        self.destroy_handle("outer_padding");
        self.destroy_handle("kile_window");
        self.destroy_handle("kile_frame");
        self.destroy_handle("layout_per_tag");
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

impl Tag {
    pub fn new() -> Tag {
        return {
            Tag {
                output: Vec::new(),
                windows: Vec::new(),
            }
        };
    }
    pub fn restore(&mut self, options: &Options) {
        for frame in &mut self.windows {
            frame.apply_padding(options.view_padding/2);
            options.push_dimensions(&frame);
        }
        options.commit();
    }
    pub fn update(&mut self, options: &mut Options) {

        let layout=match &options.layout_per_tag[options.tagmask as usize] {
            Some(layout) => { layout.clone() }
            None => {
                options.default_layout.clone()
            }
        };

        if options.view_amount > 0 {

            let total_views=options.total_views(layout.windows.len() as u32);
            Frame::from(options).new_layout(options, layout.frame, total_views, &mut self.output);

            let main_amount = options.main_amount(total_views);
            let main_view=if main_amount>0{1} else {0};

            let mut reste = (options.view_amount - main_view * main_amount) % (total_views - main_view);
            let slave_count = if total_views > 1 {
                (options.view_amount - main_view * main_amount) / (total_views - main_view)
            } else {
                options.view_amount
            };

            let mut i = 0; for frame in &self.output {
                let client_count=if i==options.main_index && main_amount > 0 {
                    main_amount
                } else {
                    if reste > 0 {
                        reste-=1;
                        slave_count+1
                    } else { slave_count }
                };
                frame.clone()
                    .new_layout(
                    options,
                    layout.windows[i as usize],
                    client_count,
                    &mut self.windows);
                i+=1;
            }
        }

        self.restore(options);
        self.clean();
    }
    pub fn clean(&mut self) {
        self.output = Vec::new();
        self.windows = Vec::new();
    }
}

impl Frame {
    pub fn from(options: &Options) -> Frame {
        let mut frame = {
            Frame {
                x: 0,
                y: 0,
                w: options.usable_width,
                h: options.usable_height,
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
        layout: (Layout, State),
        client_count: u32,
        list: &mut Vec<Frame>,
    ) {
        let (layout, state) = layout;
        let mut is_main = 0;

        if client_count > 0 {
            match layout {
                Layout::Tab => {
                    for _i in 0..client_count {
                        list.push(self.clone());
                        self.h -= 30;
                        self.y += 30;
                    }
                }
                Layout::Horizontal => {
                    let mut slave_area = self.clone();
                    let mut main_area = self.clone();
                    let reste = self.h % client_count;
                    if state == State::Frame {
                        main_area.h = if options.main_amount > 0
                            && options.main_amount < options.view_amount
                            && options.main_index < client_count
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
                            && options.main_amount > 0
                            && options.main_amount < options.view_amount
                            && options.main_index < options.view_amount
                        {
                            self.h = main_area.h;
                        } else {
                            self.h = slave_area.h / (client_count - is_main);
                        }
                        if i == 0 {
                            self.h += reste;
                        }

                        list.push(*self);
                        self.y += self.h;
                    }
                }
                Layout::Vertical => {
                    let mut slave_area = self.clone();
                    let mut main_area = self.clone();
                    let reste = self.w % client_count;
                    if state == State::Frame {
                        main_area.w = if options.main_amount > 0
                            && options.main_amount < options.view_amount
                            && options.main_index < client_count
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
                            && options.main_amount > 0
                            && options.main_amount < options.view_amount
                            && options.main_index < options.view_amount
                        {
                            self.w = main_area.w;
                        } else {
                            self.w = slave_area.w / (client_count - is_main);
                        }
                        if i == 0 {
                            self.w += reste;
                        }

                        list.push(*self);
                        self.x += self.w;
                    }
                }
                Layout::Full => {
                    for _i in 0..client_count {
                        list.push(*self);
                    }
                }
            }
        }
    }
}
