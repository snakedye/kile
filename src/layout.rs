use crate::base::Frame;

pub fn layout(frame_tree:Vec<Frame>,self:&Frame) -> Vec<Frame> {

    let client_count:u32=self.client_count;
    let main_count:u32=self.main_count;
    let main_factor:f32=self.main_factor;
    let layout:Layout=self.layout;

    let mut frame:Frame:copy();

    match layout {
        tab => {
            for _i in 0..client_count {
                frame_tree.push(frame);
                if client_count > 1 {
                    frame.h-=30;
                    frame.y+=30;
                }
            }
        }
        vertical => {
            let master_height:u32= if main_count > 0 {
                self.h*((main_factor * 100.0) as u32)/(50*client_count)
            } else { 0 };

            let slave_height:u32= if main_count > 0 && client_count > 1 {
                (self.h-master_height)/(client_count-1)
            } else if main_count < 1 && client_count > 0 {
                (self.h-master_height)/(client_count)
            } else { self.h };

            for i in 0..client_count {
                
                if client_count > 1 {
                    frame.h= if i+1==main_count {
                        master_height
                    } else if i < client_count-1 {
                        slave_height
                    } else {
                        self.y+self.h-frame.y
                    }
                }
                frame_tree.push(frame);
                frame.y+=frame.h;
           }
        }
        horizontal => {
            let master_width:u32= if main_count > 0 {
                self.w*((main_factor * 100.0) as u32)/(50*client_count)
            } else { 0 };

            let slave_width:u32= if main_count > 0 && client_count > 1 {
                (self.w-master_width)/(client_count-1)
            } else if main_count < 1 && client_count > 0 {
                (self.w-master_width)/(client_count)
            } else { self.w };

            for i in 0..client_count {
                
                if client_count > 1 {
                    frame.w= if i+1==main_count {
                        master_width
                    } else if i < client_count-1 {
                        slave_width
                    } else {
                        self.x+self.w-frame.x
                    }
                }
                frame_tree.push(frame);
                frame.x+=frame.w;
           }
        }
        full => {
            for _i in 0..client_count {
                frame_tree.push(self);
            }
        }
        _ => {
            println!("{} isn't a valid layout", layout);
            std::process::exit(0);
        }
    }

    frame_tree
}
