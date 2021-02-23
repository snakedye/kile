use crate::layout::layout;

#[derive(Copy, Clone, Debug)]
pub enum Layout {
    Vertical,
    Horizontal,
    Dwindle,
    Center,
    Tab,
    Full,
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
    pub main_count: u32,
    pub main_index: u32,
    pub main_factor: f32,
    pub client_count: u32,
    pub state: State,
    pub layout: Layout
}

impl Frame {
    pub fn new(x:u32,y:u32,w:u32,h:u32,client_count:u32,main_count:u32,main_index:u32,main_factor:f32,state:State,layout:Layout) -> Frame {
        return { Frame {
            x: x,
            y: y,
            w: w,
            h: h,
            client_count: client_count,
            main_count: main_count,
            main_index: main_index,
            main_factor: main_factor,
            state: state,
            layout: layout,
        }}
    }
    pub fn copy(&self) -> Frame {
        *self
    }
    pub fn generate(&self,window_tree:&mut Vec<Frame>) {
        layout(&self,window_tree);
    }
    pub fn set_layout(&mut self, layout:&str) {
        match layout {
            "tab" => (*self).layout=Layout::Tab,
            "ver" => (*self).layout=Layout::Vertical,
            "hor" => (*self).layout=Layout::Horizontal,
            "cen" => (*self).layout=Layout::Center,
            "dwd" => (*self).layout=Layout::Dwindle,
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
        if main_count >= 0 && main_count < self.client_count {
            (*self).main_count = main_count;
        }
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
    pub fn set_state(&mut self, state:State) {
        (*self).state = state;
    }
    pub fn set_main(&mut self) {
        (*self).state = State::Main;
    }
    pub fn set_slave(&mut self) {
        (*self).state = State::Slave;
    }
    pub fn validate(&self) {
        if self.w == 0 || self.h == 0 || self.client_count == 0 || self.main_count >= self.client_count || self.main_factor == 0.0 {
            println!("Invalid arguments");
            std::process::exit(0);
        }
    }
}
