use super::layout::layout;

#[derive(Copy, Clone, Debug)]
pub enum Layout {
    Tab,
    Full,
    Dwindle,
    Vertical,
    Horizontal,
    DwindleMod,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum State {
    Main,
    Slave
}

#[derive(Copy, Clone, Debug)]
pub struct Frame{
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
    main_count: u32,
    main_index: u32,
    main_factor: f32,
    client_count: u32,
    state: State,
    layout: Layout
}

impl Frame {
    pub fn new() -> Frame {
        return { Frame {
            x: 0,
            y: 0,
            w: 0,
            h: 0,
            client_count: 0,
            main_count: 0,
            main_index: 0,
            main_factor: 0.5,
            state: State::Slave,
            layout: Layout::Full,
        }}
    }
    pub fn copy(&self) -> Frame {
        *self
    }
    pub fn generate(&mut self,window_tree:&mut Vec<Frame>) {
        layout(self,window_tree);
    }
    pub fn set_layout(&mut self, layout:&str) {
        match layout {
            "tab" => (*self).layout=Layout::Tab,
            "ver" => (*self).layout=Layout::Vertical,
            "hor" => (*self).layout=Layout::Horizontal,
            "dwd" => (*self).layout=Layout::Dwindle,
            "dwm" => (*self).layout=Layout::DwindleMod,
            "ful" => (*self).layout=Layout::Full,
            _ => {
                println!("{} isn't a valid layout", layout);
                std::process::exit(0);
            }
        }
    }
    pub fn set_client_count(&mut self, client_count:u32) {
        (*self).client_count = client_count;
    }
    pub fn set_main_count(&mut self, main_count:u32) {
        (*self).main_count = main_count;
    }
    pub fn set_main_index(&mut self, main_index:u32) {
        if main_index < self.client_count {
            (*self).main_index = main_index;
        }
    }
    pub fn set_main_factor(&mut self, main_factor:f32) {
        if main_factor > 0.0 && main_factor < 1.0 {
            (*self).main_factor = main_factor;
        }
    }
    pub fn set_main(&mut self) {
        (*self).state = State::Main;
    }
    pub fn set_slave(&mut self) {
        (*self).state = State::Slave;
    }
    pub fn get_client_count(&mut self)->u32 {
        (*self).client_count
    }
    pub fn get_main_count(&mut self)->u32 {
        (*self).main_count
    }
    pub fn get_main_index(&mut self)->u32 {
        (*self).main_index
    }
    pub fn get_main_factor(&mut self)->f32 {
        (*self).main_factor
    }
    pub fn get_layout(&mut self)->Layout {
        (*self).layout
    }
    pub fn is_main(&mut self)->bool {
        if self.state == State::Main {
            return true
        }
        false
    }
    pub fn fix(&mut self) {
        if self.main_index >= self.client_count && self.client_count > 0 {
            (*self).main_index = self.client_count-1;
        }
        if self.main_count >= self.client_count && self.client_count > 0 {
            (*self).main_count = self.client_count-1;
        }
    }
    pub fn validate(&self) {
        if self.w == 0 || self.h == 0 || self.client_count == 0 || self.main_count > self.client_count || self.main_factor == 0.0 {
            println!("Invalid arguments");
            std::process::exit(0);
        }
    }
}
