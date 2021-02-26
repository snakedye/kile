use crate::base::Frame;

pub fn dwindle(window_tree:&mut Vec<Frame>, mut output:Frame, modi:u32) {

    let mut buffer:Vec<Frame>=Vec::new();
    let client_count:u32=output.client_count;

    for i in 0..client_count {
        let mut current:Frame=if i==0 {
            output.set_main();
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
}

pub fn combi(window_tree:&mut Vec<Frame>, layouts:Vec<&str>, mut output:Frame) {

    let mut client_count:u32=output.client_count;
    let main_count:u32=output.main_count;

    output.set_layout(layouts[0]);
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

    // println!("{}",output.client_count);
    // println!("{}",output.main_count);

    let mut main_tree:Vec<Frame>=Vec::new();
    output.fix();
    output.generate(&mut main_tree);

    // It only serves to know if a slave window has been added to the tree
    let mut fuck:u32=1;

    for i in 0..output.client_count {
        main_tree[i as usize].set_layout(layouts[(i+1) as usize]);
        if i==output.main_index && output.main_count!=0 {
            // println!("{}",output.main_count);
            main_tree[i as usize].set_client_count(main_count);
        } else if output.client_count > 1 {
            let mut count:u32=(client_count-main_count)/(output.client_count-1);
            // println!("{}",client_count-main_count);
            if fuck==1 && count != 0 {
                if count*(output.client_count-1) >= client_count-main_count {
                    count+=(count*(output.client_count-1))-(client_count-main_count);
                } else {
                    count+=client_count-main_count-(count*(output.client_count-1));
                }
                // println!("{}",count);
                client_count-=count;
                output.client_count-=1;
                fuck=0;
            }
            main_tree[i as usize].set_client_count(count);
        } else {
            main_tree[i as usize].set_client_count(client_count);
        }
        // println!("{}",main_tree[i as usize].client_count);
        main_tree[i as usize].generate(window_tree);
    }
}

pub fn center(window_tree:&mut Vec<Frame>, mut output:Frame) {

    let layouts:Vec<&str>=["ver","hor","hor","hor"].to_vec();
    output.set_main_index(1);
    combi(window_tree,layouts,output);
}
