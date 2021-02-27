use crate::frame::Frame;

pub fn combi(window_tree:&mut Vec<Frame>, layouts:Vec<&str>, mut output:Frame) {

    let mut client_count:u32=output.get_client_count();
    let main_count:u32=output.get_main_count();

    output.set_layout(layouts[0]);

    // Generate the correct amount of frames
    output.set_client_count(if client_count < layouts.len() as u32 {
        client_count
    } else { 
        if main_count<1 {
            (layouts.len()-2) as u32
        } else if client_count < layouts.len() as u32 {
            client_count
        } else {
            if client_count-main_count+1 < (layouts.len()-1) as u32 {
                (layouts.len() as u32)-(client_count-main_count)
            } else {
                (layouts.len()-1) as u32
            }
        }
    });

    let mut main_tree:Vec<Frame>=Vec::new();
    output.fix();
    output.generate(&mut main_tree);

    // It only serves to know if a slave window has been added to the tree
    let mut fuck:u32=1;

    for i in 0..output.get_client_count() {

        main_tree[i as usize].set_layout(layouts[(i+1) as usize]);

        if output.is_main() && output.get_main_count()!=0 {

            main_tree[i as usize].set_client_count(main_count);
        } else if output.get_client_count() > 1 {

            let mut count:u32=(client_count-main_count)/(output.get_client_count()-1);

            if fuck==1 && count != 0 {
                if count*(output.get_client_count()-1) >= client_count-main_count {
                    count+=(count*(output.get_client_count()-1))-(client_count-main_count);
                } else {
                    count+=client_count-main_count-(count*(output.get_client_count()-1));
                }
                client_count-=count;
                let output_client_count=output.get_client_count()-1;
                output.set_client_count(output_client_count);
                fuck=0;
            }

            main_tree[i as usize].set_client_count(count);
        } else {

            main_tree[i as usize].set_client_count(client_count);
        }

        main_tree[i as usize].generate(window_tree);
    }
}
