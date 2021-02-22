use crate::base::Frame;
use crate::layout::layout;

pub fn master_stack(mut window_tree:Vec<Frame>, frame:Frame) -> Vec<base::Frame> {

    let main_tree:Vec<base::Frame>=frame.generate();

    for frame in main_frame {
        window_tree=frame.set_layout(Layout::horizontal).set_client_count().generate(window_tree);
    }

    window_tree
}

