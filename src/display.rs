use super::options::{Layout, Options};
use crate::wayland::{
    river_layout_unstable_v1::zriver_layout_manager_v1::ZriverLayoutManagerV1,
    river_options_unstable_v1::zriver_options_manager_v1::ZriverOptionsManagerV1,
    river_status_unstable_v1::zriver_status_manager_v1::ZriverStatusManagerV1,
};
use wayland_client::protocol::{wl_output::WlOutput, wl_seat::WlSeat};
use wayland_client::Main;

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
}

#[derive(Clone, Debug)]
pub struct Tag {
    pub output: Layout,
    pub frames: Vec<Layout>,
}

#[derive(Copy, Clone, Debug)]
pub struct Rectangle {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

pub struct Frame {
    pub output: bool,
    pub layout: Layout,
    pub client_count: u32,
    pub rect: Option<Rectangle>,
    pub windows: Vec<Rectangle>,
}

impl Context {
    pub fn new(namespace: String) -> Context {
        return {
            Context {
                running: false,
                namespace: namespace,
                options: Options::new(),
                layout_manager: None,
                options_manager: None,
                status_manager: None,
                output: None,
                seat: None,
            }
        };
    }
    pub fn init(&mut self) {
        self.options.update(self.clone());
    }
    pub fn update(&mut self) {
        if !self.running {
            self.destroy();
        }
        if self.options.view_amount > 0 {
            match self.options.focused() {
                Some(mut tag) => tag.update(&mut self.options),
                None => {
                    let mut tag = self.options.default_layout.clone();
                    tag.update(&mut self.options);
                    // self.options.set_focused(tag);
                }
            };
        }
    }
    pub fn destroy(&mut self) {
        self.status_manager.as_ref().unwrap().destroy();
        self.layout_manager.as_ref().unwrap().destroy();
        self.options_manager.as_ref().unwrap().destroy();
    }
}

impl Tag {
    pub fn new() -> Tag {
        {
            Tag {
                output: Layout::Full,
                frames: Vec::new(),
            }
        }
    }
    pub fn update(&mut self, options: &mut Options) {
        let total_views = options.total_views(self.frames.len() as u32);

        let mut output = Frame::from(self.output, total_views, None, true);
        output.generate(options);

        let main_amount = options.main_amount(total_views);
        let slave_amount;
        let mut reste = if main_amount > 0 {
            slave_amount = (options.view_amount - main_amount) / (output.client_count - 1);
            (options.view_amount - main_amount) % (total_views - 1)
        } else {
            slave_amount = options.view_amount / output.client_count;
            options.view_amount % total_views
        };

        let mut i = 0;
        for rect in output.windows {
            let mut iter = self.frames.iter();
            let layout=match iter.next() {
                Some(layout)=>layout,
                None=>&Layout::Full
            };
            if i == options.main_index && main_amount > 0 {
                Frame::from(*layout, main_amount, Some(rect), false)
                    .generate(options);
            } else {
                let client_count = if reste > 0 {
                    reste -= 1;
                    slave_amount + 1
                } else {
                    slave_amount
                };
                Frame::from(*layout, client_count, Some(rect), false)
                    .generate(options);
            }
            i += 1;
        }

        options.commit();
    }
}

impl Rectangle {
    pub fn from(options: &Options) -> Rectangle {
        {
            Rectangle {
                x: 0,
                y: 0,
                w: options.usable_width,
                h: options.usable_height,
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

impl Frame {
    pub fn from(layout: Layout, client_count: u32, rect: Option<Rectangle>, output: bool) -> Frame {
        {
            Frame {
                output: output,
                layout: layout,
                client_count: client_count,
                rect: rect,
                windows: Vec::new(),
            }
        }
    }
    pub fn push_dimensions(&mut self, options: &Options) {
        for window in &mut self.windows {
            if !options.smart_padding || options.view_amount > 1 {
                window.apply_padding(options.view_padding / 2);
            }
            options.push_dimensions(&window);
        }
    }
    pub fn generate(&mut self, options: &Options) {
        let mut rect = match self.rect {
            Some(rect) => rect,
            None => {
                let mut rect = Rectangle::from(options);
                if !options.smart_padding || (self.output && options.view_amount > 1) {
                    rect.apply_padding(options.outer_padding);
                }
                rect
            }
        };

        if self.client_count > 0 {
            match self.layout {
                Layout::Tab => {
                    for _i in 0..self.client_count {
                        self.windows.push(rect);
                        rect.h -= 30;
                        rect.y += 30;
                    }
                }
                Layout::Horizontal => {
                    let mut slave_area = rect;
                    let mut main_area = rect;
                    let reste = rect.h % self.client_count;
                    if self.output {
                        main_area.h = if options.main_amount > 0
                            && self.client_count > 1
                            && options.main_amount < options.view_amount
                            && options.main_index < self.client_count
                        {
                            (rect.h * (options.main_factor * 100.0) as u32)
                                / (50 * self.client_count)
                        } else {
                            0
                        };
                        slave_area.h -= main_area.h;
                    }
                    for i in 0..self.client_count {
                        if self.output && i == options.main_index && main_area.h > 0 {
                            rect.h = main_area.h;
                        } else if self.output && main_area.h > 0 {
                            rect.h = slave_area.h / (self.client_count - 1);
                        } else {
                            rect.h = slave_area.h / self.client_count;
                        }
                        if i == 0 {
                            rect.h += reste;
                        }

                        self.windows.push(rect);
                        rect.y += rect.h;
                    }
                }
                Layout::Vertical => {
                    let mut slave_area = rect;
                    let mut main_area = rect;
                    let reste = rect.w % self.client_count;
                    if self.output {
                        main_area.w = if options.main_amount > 0
                            && self.client_count > 1
                            && options.main_amount < options.view_amount
                            && options.main_index < self.client_count
                        {
                            (rect.w * (options.main_factor * 100.0) as u32)
                                / (50 * self.client_count)
                        } else {
                            0
                        };
                        slave_area.w -= main_area.w;
                    }
                    for i in 0..self.client_count {
                        if self.output && i == options.main_index && main_area.w > 0 {
                            rect.w = main_area.w;
                        } else if self.output && main_area.w > 0 {
                            rect.w = slave_area.w / (self.client_count - 1);
                        } else {
                            rect.w = slave_area.w / self.client_count;
                        }
                        if i == 0 {
                            rect.w += reste;
                        }

                        self.windows.push(rect);
                        rect.x += rect.w;
                    }
                }
                Layout::Full => {
                    for _i in 0..self.client_count {
                        self.windows.push(rect);
                    }
                }
            }
            if !self.output {
                self.push_dimensions(options);
            }
        }
    }
}
