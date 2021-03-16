use super::layout::engine;
use super::options::Layout;
use super::options::Options;
use wayland_client::protocol::wl_output::WlOutput;
use crate::wayland::{
    river_layout_unstable_v1::{
        zriver_layout_manager_v1::ZriverLayoutManagerV1,
        zriver_layout_v1::ZriverLayoutV1,
    },
    river_options_unstable_v1::{
        zriver_options_manager_v1::ZriverOptionsManagerV1,
    },
};
use wayland_client::Main;

#[derive(Copy, Clone, Debug)]
pub enum State {
    Main,
    Slave,
    Output,
}

#[derive(Clone, Debug)]
pub struct Context {
    pub namespace: String, // Namespace is either manual or dynamic
    pub layout_manager:Option<Main<ZriverLayoutManagerV1>>,
    pub options_manager:Option<Main<ZriverOptionsManagerV1>>,
    pub output: WlOutput,
    pub tags:Vec<Tag>, // It's the amount of possible tags, it might be too much to set all possible tags
    pub focused: u32,
}

#[derive(Clone, Debug)]
pub struct Tag {
    pub tagmask:u32,
    pub serial:u32,
    pub main_frame:Frame,
    pub layout:Main<ZriverLayoutV1>,
    pub client_count: u32,
    pub windows:Vec<Frame>,
    pub modifier:Vec<Layout>,
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

impl Context {
    pub fn new(output:WlOutput, namespace:String)->Context {
        return { Context {
            layout_manager: None,
            options_manager: None,
            namespace: namespace,
            output: output,
            focused: 0,
            tags:Vec::with_capacity(256),
        } }
    }
    pub fn init(&mut self) {
        let options=Options::new(self.clone());
        self.tags.push(Tag::new(self, &options));
        self.update_focus(options.tagmask);
    }
    pub fn update(&mut self) {
        let options=Options::new(self.clone());
        let index=tag_index(&options.tagmask);
        let mut tag=self.tags[index].clone();
        if self.focused != options.tagmask {
            (*self).focused=options.tagmask;
            if tag.is_set() /*&& tag.layout==options.arguments*/ {
                tag.restore(&options);
            } else {
                self.tags[index]=Tag::new(self, &options);
            }
        } else {
            // options.modifier=tag.layout;
            tag.update(&options);
        }
    }
    pub fn update_focus(&mut self,tagmask:u32) {
        self.focused=tagmask;
    }
    pub fn destroy(&self, output: &WlOutput) {
        self.options_manager.clone().unwrap().destroy();
        self.tags[self.focused as usize].layout.destroy();
        self.destroy_handle("main-index", output);
        self.destroy_handle("main-count", output);
        self.destroy_handle("main-factor", output);
        self.destroy_handle("view-padding", output);
        self.destroy_handle("output-padding", output);
    }
    pub fn destroy_handle(&self, name:&str, output: &WlOutput) {
        let option = self.options_manager
            .clone()
            .unwrap()
            .get_option_handle(String::from(name), Some(output));
        option.destroy();
    }
}

pub fn tag_index(tagmask:&u32)->usize {
    usize::from_str_radix(&tagmask.to_string(), 2).unwrap()
}

impl Tag {
    pub fn new(context:&Context, options:&Options)->Tag {
        return { Tag {
            // tagmask:options.tagmask,
            // 0 in the meanwhile because no river-status
            tagmask:0,
            layout:context.layout_manager
            .clone()
            .unwrap()
            .get_river_layout(&context.output, context.namespace.clone()),
            serial:options.serial,
            main_frame:Frame::new(options),
            client_count:0,
            modifier:options.arguments.clone(),
            windows:Vec::new(),
        } }
    }
    pub fn restore(&self,options:&Options) {
        for frame in &self.windows {
            self.push_dimensions(&frame);
        }
        self.commit();
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
    pub fn push_dimensions(&self, frame:&Frame) {
        self.layout.push_view_dimensions(
            self.serial,
            frame.x as i32,
            frame.y as i32,
            frame.w,
            frame.h,
        )
    }
    pub fn commit(&self) {
        self.layout.commit(self.serial);
    }
}

impl Frame {
    pub fn new(options:&Options) -> Frame {
        let mut frame={ Frame {
            x: 0,
            y: 0,
            index: 0,
            w: options.usable_width,
            h: options.usable_height,
            state: State::Slave,
        }};
        frame.apply_padding(&options.output_padding);
        frame
    }
    pub fn set_main(mut self) {
        self.state = State::Main;
    }
    pub fn set_slave(mut self) {
        self.state = State::Slave;
    }
    pub fn apply_padding(&mut self,padding:&u32) {
        self.x+=padding;
        self.y+=padding;
        self.w+=2*padding;
        self.h+=2*padding;
    }
}
