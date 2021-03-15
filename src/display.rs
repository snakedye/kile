use super::layout::engine;
use super::options::Layout;
use super::options::Options;
use wayland_client::protocol::wl_output::WlOutput;

#[derive(Copy, Clone, Debug)]
pub enum State {
    Main,
    Slave,
    Output,
}

pub struct Output {
    pub namespace: String, // Namespace is either manual or dynamic
    pub output: WlOutput,
    pub tags:Vec<Tag>, // It's the amount of possible tags, it might be too much to set all possible tags
    pub focused: u32,
}

#[derive(Clone, Debug)]
pub struct Tag {
    pub tagmask:u32,
    pub main_frame:Frame,
    pub client_count: u32,
    pub windows:Vec<Frame>,
    pub layout:Vec<Layout>,
}

#[derive(Copy, Clone, Debug)]
pub struct Frame{
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
    pub index: u32,
    state: State,
}

impl Output {
    pub fn new(output:WlOutput)->Output {
        return { Output {
            namespace: "Something".to_string(),
            output: output,
            focused: 0,
            tags:Vec::new(),
        } }
    }
    pub fn init(&mut self) {
        let options=Options::new(&self.output);
        self.focused=options.tagmask;
    }
    pub fn update(&mut self) {
        let options=Options::new(&self.output);
        let index=tag_index(&options.tagmask);
        let mut tag=self.tags[index].clone();
        if self.focused != options.tagmask {
            self.focused=options.tagmask;
            if tag.is_set() /*&& tag.layout==options.arguments*/ {
                tag.restore(&options);
            } else {
                self.tags[index]=Tag::new(&options);
            }
        } else {
            // options.layout=tag.layout;
            tag.update(&options);
        }
    }
}

pub fn tag_index(tagmask:&u32)->usize {
    usize::from_str_radix(&tagmask.to_string(), 2).unwrap()
}

impl Tag {
    pub fn new(options:&Options)->Tag {
        return { Tag {
            // tagmask:options.tagmask,
            // 0 in the meanwhile because no river-status
            tagmask:0,
            main_frame:Frame::new(options),
            client_count:0,
            layout:options.arguments.clone(),
            windows:Vec::new(),
        } }
    }
    pub fn restore(&self,options:&Options) {
        for frame in &self.windows {
            options.push_dimensions(&frame);
        }
        options.commit();
    }
    pub fn is_set(&self)->bool {
        if self.client_count>0 {
            return true
        }
        false
    }
    pub fn update(&mut self, options:&Options) {
        if options.view_amount > self.client_count {
            self.generate(options);
        } else {
            for i in self.client_count..options.view_amount {
                self.windows.remove(i as usize);
            }
            self.generate(&options);
            self.restore(&options)
        }
    }
    pub fn generate(&mut self, options:&Options) {
        engine(self, options);
    }
}

impl Frame {
    pub fn new(options:&Options) -> Frame {
        return { Frame {
            x: 0,
            y: 0,
            index: 0,
            w: options.usable_width,
            h: options.usable_height,
            state: State::Slave,
        }}.apply_padding(&options)
    }
    pub fn set_main(mut self) {
        self.state = State::Main;
    }
    pub fn set_slave(mut self) {
        self.state = State::Slave;
    }
    pub fn apply_padding(mut self,options:&Options)->Frame {
        self.x+=options.output_padding;
        self.y+=options.output_padding;
        self.w+=2*options.output_padding;
        self.h+=2*options.output_padding;
        self
    }
}
