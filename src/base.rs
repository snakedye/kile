use crate::layout::layout;

#[derive(Copy, Clone, Debug)]
pub enum Layout {
    Vertical,
    Horizontal,
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
    pub fn generate(&self,window_tree:Vec<Frame>) -> Vec<Frame> {
        layout(&self,window_tree)
    }
    // pub fn set_layout(mut self, layout:Layout) {
    //     self.layout = layout;
    // }
    // pub fn set_client_count(mut self, client_count:u32) {
    //     self.client_count = client_count;
    // }
    // pub fn set_main_count(mut self, main_count:u32) {
    //     self.main_count = main_count;
    // }
    // pub fn set_main_factor(mut self, main_factor:f32) {
    //     self.main_factor = main_factor;
    // }
}
