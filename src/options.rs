use super::display::Rectangle;
use crate::wayland::river_layout_unstable_v1::zriver_layout_v1::ZriverLayoutV1;
use wayland_client::Main;

#[derive(Clone, Debug)]
pub struct Options {
    pub serial: u32,
    pub tagmask: u32,
    pub zlayout: Option<Main<ZriverLayoutV1>>,
    pub view_amount: u32,
    pub usable_width: u32,
    pub usable_height: u32,
    pub smart_padding: bool,
    pub view_padding: u32,
    pub outer_padding: u32,
    pub xoffset: i32,
    pub yoffset: i32,
    pub main_index: u32,
    pub main_factor: f64,
    pub main_amount: u32,
}

#[derive(Copy, Clone, Debug)]
pub enum Layout {
    Tab,
    Full,
    Vertical,
    Recursive { modi: u32, index: Option<usize> },
    Horizontal,
}

impl Options {
    pub fn new() -> Options {
        return {
            Options {
                serial: 0,
                tagmask: 0,
                zlayout: None,
                view_amount: 0,
                smart_padding: false,
                view_padding: 0,
                outer_padding: 0,
                xoffset: 0,
                yoffset: 0,
                usable_width: 0,
                usable_height: 0,
                main_factor: 0.0,
                main_index: 0,
                main_amount: 0,
            }
        };
    }
    pub fn frames_amount(&self, frames_available: u32) -> u32 {
        if self.main_amount >= self.view_amount {
            1
        } else if self.view_amount >= frames_available {
            if 1 + self.view_amount - self.main_amount < frames_available
                && self.main_amount > self.view_amount / frames_available
            {
                1 + self.view_amount - self.main_amount
            } else {
                frames_available
            }
        } else {
            self.view_amount
        }
    }
    pub fn main_amount(&self, frames_amount: u32) -> u32 {
        if self.main_index + self.main_amount <= self.view_amount
            && frames_amount > 1
            && self.main_amount > 0
        {
            if self.main_index + self.main_amount > self.view_amount {
                self.view_amount - self.main_index
            } else {
                self.main_amount
            }
        } else {
            0
        }
    }
    pub fn outer_layout(layout_output: String) -> Layout {
        match layout_output.chars().next() {
            Some(c) => Options::layout(c),
            None => Layout::Full,
        }
    }
    pub fn usable_width(&self) -> u32 {
        if self.xoffset != 0 || self.xoffset < self.usable_width as i32 {
            if self.xoffset > 0 {
                ((self.usable_width as i32) - self.xoffset) as u32
            } else {
                ((self.usable_width as i32) + self.xoffset) as u32
            }
        } else {
            self.usable_width
        }
    }
    pub fn usable_height(&self) -> u32 {
        if self.yoffset != 0 || self.yoffset < self.usable_height as i32 {
            if self.yoffset > 0 {
                ((self.usable_height as i32) - self.yoffset) as u32
            } else {
                ((self.usable_height as i32) + self.yoffset) as u32
            }
        } else {
            self.usable_height
        }
    }
    pub fn inner_layout(string: String) -> Vec<Layout> {
        let mut layout = Vec::new();

        for c in string.chars() {
            layout.push(Options::layout(c));
        }

        layout
    }
    fn layout(c: char) -> Layout {
        match c {
            'v' => Layout::Vertical,
            'h' => Layout::Horizontal,
            't' => Layout::Tab,
            'd' => Layout::Recursive {
                modi: 0,
                index: None,
            },
            'D' => Layout::Recursive {
                modi: 1,
                index: None,
            },
            'r' => Layout::Recursive {
                modi: 0,
                index: Some(0),
            },
            'R' => Layout::Recursive {
                modi: 1,
                index: Some(0),
            },
            'f' => Layout::Full,
            _ => {
                println!("{}: Not a valid character at index", c);
                Layout::Full
            }
        }
    }
    pub fn push_dimensions(&self, rect: &Rectangle) {
        self.zlayout.as_ref().unwrap().push_view_dimensions(
            self.serial,
            rect.x as i32,
            rect.y as i32,
            rect.w,
            rect.h,
        )
    }
    pub fn debug(&self) {
        println!("Options - {}", self.serial);
        println!("\n  ZriverLayoutV1");
        println!("    view_amount : {}", self.view_amount);
        println!("    usable_width : {}", self.usable_width);
        println!("    usable_height : {}", self.usable_height);
        println!("\n  ZriverOptionHandleV1");
        println!("    outer_padding : {}", self.outer_padding);
        println!("    view_padding : {}", self.view_padding);
        println!("    xoffset : {}", self.xoffset);
        println!("    yoffset : {}", self.yoffset);
        println!("    smart_padding : {}", self.smart_padding);
        println!("    main_factor : {}", self.main_factor);
        println!("    main_index : {}", self.main_index);
        println!("    main_amount : {}", self.main_amount);
        println!("\n  ZriverOutputStatusV1");
        println!("    tagmask : {}\n", self.tagmask);
    }
    pub fn commit(&self) {
        self.zlayout.as_ref().unwrap().commit(self.serial);
    }
}
