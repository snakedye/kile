#[derive(Copy, Clone)]
pub struct Parameters{
    pub clients_count: u32,
    pub main_count: u32,
    pub main_factor: f32,
}

#[derive(Copy, Clone)]
pub struct Frame{
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
    pub state: State
}

// Defines the state of a frame
#[derive(Copy, Clone)]
pub enum State {
    Full{ capacity: u32 },
    Free,
}

// Frame constructor
pub fn new_frame(x:u32,y:u32,w:u32,h:u32,state:State) -> Frame {
    return { Frame {
        x: x,
        y: y,
        w: w,
        h: h,
        state: state
    }}
}

