use crate::base::Frame;
use crate::base::Layout;
use crate::base::State;

pub fn layout(chosen:&Frame, mut frame_tree:Vec<Frame>) -> Vec<Frame> {

    let client_count:u32=chosen.client_count;
    let main_index:u32=chosen.main_index;
    let main_count:u32=chosen.main_count;
    let main_factor:f32=chosen.main_factor;
    let layout:Layout=chosen.layout;

    let mut frame:Frame=chosen.copy();

    match layout {
        Layout::Tab => {
            for _i in 0..client_count {
                frame_tree.push(frame);
                if client_count > 1 {
                    frame.h-=30;
                    frame.y+=30;
                }
            }
        }
        Layout::Horizontal => {
            let master_height:u32= if frame.state == State::Main {
                chosen.h*((main_factor * 100.0) as u32)/(50*client_count)
            } else { 0 };

            let slave_height:u32= if frame.state == State::Main && client_count > 1 {
                (chosen.h-master_height)/(client_count-1)
            } else if frame.state == State::Slave && client_count > 0 {
                (chosen.h-master_height)/(client_count)
            } else { chosen.h };

            for i in 0..client_count {
                
                if client_count > 1 {
                    frame.h=if i==main_index && frame.state == State::Main {
                        master_height
                    } else if i < client_count-1 {
                        slave_height
                    } else {
                        chosen.y+chosen.h-frame.y
                    }
                }
                frame_tree.push(frame);
                frame.y+=frame.h;
           }
        }
        Layout::Vertical => {
            let master_width:u32=if frame.state == State::Main {
                chosen.w*((main_factor * 100.0) as u32)/(50*client_count)
            } else { 0 };

            let slave_width:u32= if frame.state == State::Main && client_count > 1 {
                (chosen.w-master_width)/(client_count-1)
            } else if frame.state == State::Slave && client_count > 0 {
                (chosen.w-master_width)/(client_count)
            } else { chosen.w };

            for i in 0..client_count {
                
                if client_count > 1 {
                    frame.w=if i==main_index && frame.state == State::Main {
                        master_width
                    } else if i < client_count-1 {
                        slave_width
                    } else {
                        chosen.x+chosen.w-frame.x
                    }
                }
                frame_tree.push(frame);
                frame.x+=frame.w;
           }
        }
        Layout::Full => {
            for _i in 0..client_count {
                frame_tree.push(frame);
            }
        }
        _ => {
            println!("{:?} isn't a valid layout", layout);
            std::process::exit(0);
        }
    }

    frame_tree[main_index as usize].state = State::Main;

    frame_tree
}
