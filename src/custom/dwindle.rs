use crate::frame::Frame;

pub fn generate(window_tree:&mut Vec<Frame>, mut output:Frame, modi:u32) {

    let mut buffer:Vec<Frame>=Vec::new();
    let client_count:u32=output.get_client_count();

    for i in 0..client_count {
        let mut current:Frame=if i==0 {
            output
        } else { buffer[buffer.len()-1] };
        if (i+modi)%2==0{
            current.set_layout("ver");
        } else {
            current.set_layout("hor");
        }
        current.set_slave();
        current.set_client_count(if i<client_count-1 {
            2
        } else { 1 });
        current.generate(&mut buffer);
        window_tree.push(buffer[(2*i) as usize]);
    }
    window_tree[output.get_main_index() as usize].set_main();
}
