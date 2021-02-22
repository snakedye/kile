use crate::base::Frame;
use crate::base::Layout;
use crate::layout::layout;

pub fn master_stack(mut window_tree:Vec<Frame>, mut output:Frame) -> Vec<Frame> {

    let client_count:u32=output.client_count;
    let main_count:u32=output.main_count;

    output.layout=Layout::Vertical;
    output.client_count=if client_count > 1 {
        2
    } else { 1 };

    let main_tree:Vec<Frame>=output.generate(window_tree.clone());

    let mut frame_count:u32=0;

    for mut frame in main_tree {
        if frame_count>0 {
            frame.client_count=client_count-main_count;
        } else {frame.client_count=main_count;}
        frame.layout=Layout::Horizontal;
        window_tree=frame.generate(window_tree);
        frame_count+=frame.client_count;
    }

    window_tree
}

