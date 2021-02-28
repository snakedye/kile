use crate::frame::Frame;
use crate::custom::zoom;

pub fn generate(window_tree:&mut Vec<Frame>, layouts:Vec<&str>, mut output:Frame) {

    let client_count:u32=output.get_client_count();
    let main_count:u32=output.get_main_count();

    output.set_layout(layouts[0]);

    output.set_client_count(
        if client_count >= (layouts.len()-1) as u32 {
            if client_count-main_count+1 < (layouts.len()-1) as u32 {
                ((layouts.len()-1) as u32)-(client_count-main_count)
            } else {
                (layouts.len()-1) as u32
            }
        } else {
            client_count
    });

    let mut main_tree:Vec<Frame>=Vec::new();
    output.fix();
    output.generate(&mut main_tree);
    zoom::generate(&mut main_tree, output.get_main_index() as usize);

    let is_main:u32=if main_count > 0 {1} else {0};
    let client_per_frame:u32=(client_count-main_count)/(output.get_client_count()-is_main);
    let mut rest:u32=(client_count-main_count)%(output.get_client_count()-is_main);

    let mut i:usize=0;

    for mut frame in main_tree {

        frame.set_layout(layouts[i+1]);

        if frame.is_main() && main_count>0 {

            frame.set_client_count(main_count);
        } else {
            if rest>0 {
                frame.set_client_count(client_per_frame+1);
                rest-=1;
            } else {
                frame.set_client_count(client_per_frame);
            }
        }
        frame.generate(window_tree);
        i+=1;
    }
}
