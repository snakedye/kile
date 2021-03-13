use super::engine::layout;
use super::options::Layout;
use super::display::Options;
use std::ptr::null;

pub struct Display {
    pub tags:[Tag;10];
    pub focused: Tag,
}

pub struct Tag {
    pub tagmask:u8,
    pub output:Frame,
}

#[derive(Copy, Clone, Debug)]
pub struct Frame{
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
    pub padding: u32,
    pub client_count: u32,
    pub capacity: u32,
    state: State,
    parent: &Frame, // add a lifetime parameter
    tree: Vec<Frame>,
}

impl Display {
    pub fn new() {
    }
    pub fn init(self) {
        let options=Options::get();
        self.focused=options.tagmask;
    }
    pub fn update(self) {
        let options=Options::listen();
        if self.focused != options.tagmask {
            if options.tagmask.is_set() {
                options.tagmask.restore();
            } else {
                self.tags[options.tagmask]=Tag::new(options);
            }
            self.focused=options.tagmask;
        }
    }
}

impl Tag {
    pub fn new(options:Options) {
        return { Tag {
            tagmask:options,
            output:Frame::new(options),
        } }
    }
    pub fn restore(self) {
        // More Wayland crap
        // Should just be sending the frame in the tag to the server
    }
    pub fn is_set(self)->bool {
        if self.frame.client_count>0 {
            true
        }
        false
    }
    pub fn update(self, options:Options) {
        if self.frame.client_count < options.client_count {
            frame.insert(options);
        } else {
            frame.remove(options);
        }
    }
}

impl Frame {
    pub fn new() -> Frame {
        return { Frame {
            x: 0,
            y: 0,
            w: 0,
            h: 0,
            padding: 10,
            capacity: 0,
            client_count: 0,
            main_factor: 0.5,
            state: State::Output,
            parent: null,
            tree: Vec::new(),
        }}
    }
    pub fn new(options:Options) -> Frame {
        let output:Frame=Frame {
            x: 0,
            y: 0,
            w: options.usable_width,
            h: options.usable_height,
            padding: options.view_padding,
            capacity: Unlimited, // To fix in Frame
            client_count: 0,
            state: State::Output,
            parent: null,
            tree: Vec::new(),
        }.apply_padding().generate(options)
    }
    pub fn insert(&self, options:Options) {
        if self.space_left == 0 {
            self.child[self.client_count-1].update(options);
        }
        self.generate(options);
    }
    pub fn space_left(&self)->u32 {
        self.capacity-self.client_count;
    }
    pub fn clone(&self) -> Frame {
        return { Frame {
            x: self.x,
            y: self.x,
            w: self.w,
            h: self.h,
            padding: self.padding,
            capacity: self.capacity,
            client_count: 0,
            state: State::Slave,
            parent: &self,
            tree: Vec::new(),
        } }
    }
    pub fn generate(self, options:Options) {
        if self.client_count>0 {
            self.tree=Vec::new();
        }
        match self.state {
            State::Output=>engine(self, options),
            _=>layout(self, options),
        }
    }
    // pub fn set_main_factor(self, main_factor:f32) {
    //     if main_factor > 0.0 && main_factor < 1.0 {
    //         self.main_factor = main_factor;
    //     }
    // }
    pub fn set_main(self) {
        self.state = State::Main;
    }
    pub fn set_slave(self) {
        self.state = State::Slave;
    }
    pub fn callback(self,options:Options) {

        if self.parent.state!=State::Output && self.parent.client_count + client_count > self.parent.capacity {
            self.parent.callback(options);
        }
        self.generate(options);
    }
    pub fn get_tree(self) -> &mut Vec<Frame> {
        return &mut self.tree;
    }
    pub fn push(self, child:Frame) {
        self.tree.push(child);
        self.client_count+=1;
    }
    pub fn apply_padding(self) {
        self.x+=self.padding;
        self.y+=self.padding;
        self.w+=2*self.padding;
        self.h+=2*self.padding;
        // self.generate(self.client_count);
    }
}
