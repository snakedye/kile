use crate::base;
use crate::layout;

pub fn chosen_layout(layout:&str,mut window_tree:Vec<base::Frame>, output:base::Frame, client_count:u32, main_count:u32, main_factor:f32) -> Vec<base::Frame> {
    match layout {
        "tab"=> window_tree=layout::tab(window_tree, output,client_count),
        "vertical"=> window_tree=layout::vertical(window_tree, output,client_count, main_count, main_factor),
        "horizontal"=> window_tree=layout::horizontal(window_tree,output, client_count, main_count, main_factor),
        "dwindle"=> window_tree=layout::dwindle(window_tree, output,client_count, 0),
        "dwindle_mod"=> window_tree=layout::dwindle(window_tree, output,client_count, 1),
        "left"=> window_tree=master_stack(window_tree,output, client_count, main_count, main_factor),
        "center"=> window_tree=centered_master(window_tree,output, client_count, main_count, main_factor),
        _ => {
            println!("{} isn't a valid layout", layout);
            std::process::exit(0);
        }
    }

    window_tree
}

pub fn master_stack(mut window_tree:Vec<base::Frame>, output:base::Frame, client_count:u32, main_count:u32, main_factor:f32) -> Vec<base::Frame> {

    let main_tree:Vec<base::Frame>=Vec::new();

    match client_count {
        1=> {
            let main_frame:Vec<base::Frame>=layout::vertical(main_tree, output, 1, 1, main_factor);
            window_tree=layout::tab(window_tree, main_frame[0], 1);
        }
        _=> {
            let main_frame:Vec<base::Frame>=layout::vertical(main_tree, output, 2, 1, main_factor);
            window_tree=layout::horizontal(window_tree, main_frame[0], main_count, 0, main_factor);
            window_tree=layout::horizontal(window_tree, main_frame[1], client_count-main_count, 0, main_factor);
        }
    }

    window_tree
}

pub fn centered_master(mut window_tree:Vec<base::Frame>, output:base::Frame, client_count:u32, main_count:u32, main_factor:f32) -> Vec<base::Frame> {

    let main_tree:Vec<base::Frame>=Vec::new();

    match client_count {
        1=> {
            let main_frame:Vec<base::Frame>=layout::vertical(main_tree, output, 1, 1, main_factor);
            window_tree=layout::tab(window_tree, main_frame[0], 1);
        }
        2=> {
            let main_frame:Vec<base::Frame>=layout::vertical(main_tree, output, 2, 1, main_factor);
            window_tree=layout::tab(window_tree, main_frame[0], 1);
            window_tree=layout::tab(window_tree, main_frame[1], 1);
        }
        _=> {
            let main_frame:Vec<base::Frame>=layout::vertical(main_tree, output, 3, 2, main_factor);
            window_tree=layout::tab(window_tree, main_frame[1], main_count);
            window_tree=layout::horizontal(window_tree, main_frame[2], client_count-main_count-((client_count-main_count)/2), 0, main_factor);
            window_tree=layout::horizontal(window_tree, main_frame[0], (client_count-main_count)/2, 0, main_factor);
        }
    };

    window_tree
}

pub fn combi(layouts:Vec<&str>, mut window_tree:Vec<base::Frame>, output:base::Frame, mut client_count:u32, main_count:u32, main_factor:f32) -> Vec<base::Frame> {

    let main_tree:Vec<base::Frame>=Vec::new();
    let default_client_count:u32=(client_count-main_count+1)/(layouts.len()-1) as u32;

    let main_frame:Vec<base::Frame>= if client_count-main_count+1 < layouts.len() as u32 {
        chosen_layout(layouts[0],main_tree.clone(), output, 1+client_count-main_count, 1, main_factor)
    } else if client_count < layouts.len() as u32 {
        chosen_layout(layouts[0],main_tree.clone(), output, client_count, 1, main_factor)
    } else {
        chosen_layout(layouts[0],main_tree.clone(), output, (layouts.len()-1) as u32, 1, main_factor)
    };

    for i in 0..main_frame.len() {
        let chosen_count:u32;
        if i == 0 && layouts.len() > 2 && main_count > 0 {
            chosen_count=main_count;
            window_tree=chosen_layout(layouts[i+1],window_tree, main_frame[i], chosen_count, 1, 0.5);
        } else {
            chosen_count= if i < main_frame.len()-1 && default_client_count < client_count {
                default_client_count
            } else { client_count };
            if chosen_count > 0 {
                window_tree=chosen_layout(layouts[i+1],window_tree,main_frame[i],chosen_count,1,0.5);
            }
        }

        client_count-=chosen_count;
    }

    window_tree
}

