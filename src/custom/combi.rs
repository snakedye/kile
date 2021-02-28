use crate::frame::Frame;

pub fn generate(window_tree:&mut Vec<Frame>, layouts:Vec<&str>, mut output:Frame) {

    let client_count:u32=output.get_client_count();
    let main_count:u32=output.get_main_count();

    output.set_layout(layouts[0]);

    // Generate the correct amount of frames
    output.set_client_count(
        if main_count<1 {
            (layouts.len()-2) as u32
        } else if client_count >= layouts.len() as u32 {
            if client_count-main_count+1 < (layouts.len()-1) as u32 {
                ((layouts.len()-1) as u32)-(client_count-main_count)
            } else {
                (layouts.len()-1) as u32
            }
        } else {
            client_count
    });

    // println!("main_count: {:?}",main_count);
    // println!("output.get_client_count(): {:?}",output.get_client_count());

    let mut main_tree:Vec<Frame>=Vec::new();
    output.fix();
    output.generate(&mut main_tree);

    // println!("{:?}",main_tree);
    // println!("{:?}",main_count);
    // println!("\n{:?}",main_tree);

    let is_main:u32=if main_count > 0 {1} else {0};
    let client_per_frame:u32=(client_count-main_count)/(output.get_client_count()-is_main);
    let mut rest:u32=(client_count-main_count)%(output.get_client_count()-is_main);

    // println!("client_per_frame: {}",client_per_frame);
    // println!("reste: {}",rest);
    // println!("output.get_main_index(): {:?}",output.get_main_index());

    for i in 0..output.get_client_count() {

        main_tree[i as usize].set_layout(layouts[(i+1) as usize]);

        // println!("main_tree[i as usize]: {:?}\n",main_tree[i as usize]);
        if main_tree[i as usize].is_main() && main_count>0 {

            // println!("main_tree[i as usize]: {:?}\n",main_tree[i as usize]);
            main_tree[i as usize].set_client_count(main_count);
        } else {
            if rest>0 {
                main_tree[i as usize].set_client_count(client_per_frame+1);
                rest-=1;
            } else {
                main_tree[i as usize].set_client_count(client_per_frame);
            }
        }
        main_tree[i as usize].generate(window_tree);
    }
}
