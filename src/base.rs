use crate::layout::layout;

#[derive(Copy, Clone)]
pub struct Frame{
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
    pub main_count: u32,
    pub main_factor: f32,
    pub client_count: u32,
    pub layout: Layout,
}

// Defines the layout of a frame
#[derive(Copy, Clone)]
pub enum Layout {
    vertical,
    horizontal,
    tab,
    full,
}

impl Frame {
    fn new(x: u32,y: u32,w: u32,h: u32,client_count: u32,main_count: u32,main_factor: f32) -> Frame {
        return { Frame {
            x: x,
            y: y,
            w: w,
            h: h,
            main_count: u32,
            main_factor: f32,
            client_count: u32,
            layout: full,
        }}
    }
    fn generate(window_tree:Vec<Frame>,&self) -> Vec<Frame> {
        layout(window_tree,&self)
    }
    fn set_layout(&self, layout:Layout) {
        *self.layout = layout;
    }
    fn set_client_count(&self, client_count:u32) {
        *self.client_count = client_count;
    }
}
