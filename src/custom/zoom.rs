use crate::frame::Frame;

pub fn generate(window_tree:&mut Vec<Frame>, index:usize) {

    let main_frame:Frame=window_tree[index];
    let top_frame:Frame=window_tree[0];

    (*window_tree)[0]=main_frame;
    (*window_tree)[index]=top_frame;
}
