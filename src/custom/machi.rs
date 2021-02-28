use crate::frame::Frame;
use crate::custom::combi;

pub fn generate(window_tree:&mut Vec<Frame>, layouts:Vec<&str>, mut output:Frame) {

    let mut client_count=output.get_client_count();
    let mut main_tree:Vec<Frame>=Vec::new();

    output.set_client_count((layouts.len()-1) as u32);
    combi::generate(&mut main_tree, layouts, output);

    let mut i:u32=0;
    while client_count > 0 {
        let count=if i+1==output.get_client_count() {
            client_count
        } else {
            1
        };
        main_tree[i as usize].set_client_count(count);
        main_tree[i as usize].generate(window_tree);
        i+=1;
        client_count-=count;
    }
}
