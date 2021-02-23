use crate::base::Frame;
use crate::base::Layout;
use crate::base::State;

pub fn dwindle(window_tree:&mut Vec<Frame>, mut output:Frame, modi:u32) {

    let mut buffer:Vec<Frame>=Vec::new();
    let client_count:u32=output.client_count;

    for i in 0..client_count {
        let mut current:Frame=if i==0 {
            output.set_main();
            output
        } else { buffer[buffer.len()-1] };
        if (i+modi)%2==0{
            current.set_layout("hor");
        } else {
            current.set_layout("ver");
        }
        current.set_slave();
        current.set_client_count(if i<client_count-1 {
            2
        } else { 1 });
        current.generate(&mut buffer);
        window_tree.push(buffer[(2*i) as usize]);
    }
}

pub fn combi(window_tree:&mut Vec<Frame>, layouts:Vec<&str>, mut output:Frame) {

    let mut client_count:u32=output.client_count;
    let main_count:u32=output.main_count;

    output.set_layout(layouts[0]);
    output.set_client_count(if client_count-main_count+1 < layouts.len() as u32 {
        client_count-main_count+1
    } else { (layouts.len()-1) as u32 });

    let mut main_tree:Vec<Frame>=Vec::new();
    output.generate(&mut main_tree);

    // It only serves to know if a slave window has been added to the tree
    let mut fuck:u32=1;

    for i in 0..output.client_count {
        main_tree[i as usize].set_layout(layouts[(i+1) as usize]);
        if i==output.main_index {
            main_tree[i as usize].set_client_count(main_count);
        } else {
            let mut count:u32=(((client_count-main_count)/(output.client_count-1)) as f32).floor() as u32;
            if fuck==1 && count != 0 {
                count+=(client_count-main_count)%count;
                client_count-=count;
                output.client_count-=1;
                fuck=0;
            }
            main_tree[i as usize].client_count=count;
        }
        main_tree[i as usize].generate(window_tree);
    }
}

pub fn center(window_tree:&mut Vec<Frame>, mut output:Frame) {

    let layouts:Vec<&str>=["ver","hor","hor","hor"].to_vec();
    output.set_main_index(1);
    combi(window_tree,layouts,output);
}
