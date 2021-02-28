use crate::frame::Frame;
use crate::frame::Layout;
use crate::custom::dwindle;

pub fn layout(view:&mut Frame, frame_tree:&mut Vec<Frame>) {

    let client_count:u32=view.get_client_count();
    let main_index:u32=view.get_main_index();
    let main_count:u32=view.get_main_count();
    let main_factor:f32=view.get_main_factor();
    let layout:Layout=view.get_layout();

    let mut frame:Frame=view.copy();

    match layout {
        Layout::Tab => {
            for _i in 0..client_count {
                (*frame_tree).push(frame);
                if client_count > 1 {
                    frame.h-=30;
                    frame.y+=30;
                }
            }
        }
        Layout::Horizontal => {
            let master_height:u32=if main_count>0 {
                view.h*((main_factor * 100.0) as u32)/(50*client_count)
            } else { 0 };

            let slave_height:u32= if main_count>0 && client_count > 1 {
                (view.h-master_height)/(client_count-1)
            } else if client_count > 0 {
                (view.h-master_height)/(client_count)
            } else { view.h };

            for i in 0..client_count {
                
                if client_count > 1 {
                    frame.h=if i==main_index && main_count>0 {
                        frame.set_main();
                        master_height
                    } else if i < client_count-1 {
                        frame.set_slave();
                        slave_height
                    } else {
                        frame.set_slave();
                        view.y+view.h-frame.y
                    }
                }
                frame.set_main_count(0);

                (*frame_tree).push(frame);
                frame.y+=frame.h;
           }
        }
        Layout::Vertical => {
            let master_width:u32=if main_count>0 {
                view.w*((main_factor * 100.0) as u32)/(50*client_count)
            } else { 0 };

            let slave_width:u32= if main_count>0 && client_count > 1 {
                (view.w-master_width)/(client_count-1)
            } else if client_count > 0 {
                (view.w-master_width)/(client_count)
            } else { view.w };

            for i in 0..client_count {
                
                if client_count > 1 {
                    frame.w=if i==main_index && main_count>0 {
                        frame.set_main();
                        master_width
                    } else if i < client_count-1 {
                        frame.set_slave();
                        slave_width
                    } else {
                        frame.set_slave();
                        view.x+view.w-frame.x
                    }
                }
                frame.set_main_count(0);

                (*frame_tree).push(frame);
                frame.x+=frame.w;
           }
        }
        Layout::Dwindle => {
            dwindle::generate(frame_tree, *view, 1);
        }
        Layout::DwindleMod => {
            dwindle::generate(frame_tree, *view, 0);
        }
        Layout::Full => {
            for _i in 0..client_count {
                (*frame_tree).push(frame);
            }
        }
    }
}
