use crate::basic;
use crate::layout;

pub fn chosen_layout(layout:&str,mut window_tree:Vec<basic::Frame>, output:basic::Frame, client_count:u32, master_count:u32, master_factor:f32) -> Vec<basic::Frame> {
    match layout {
        "tab"=> window_tree=basic::tab(window_tree,client_count, output),
        "vertical"=> window_tree=basic::vertical(window_tree,client_count, master_count, master_factor, output),
        "horizontal"=> window_tree=basic::horizontal(window_tree, client_count, master_count, master_factor,output),
        "dwindle"=> window_tree=basic::dwindle(window_tree,client_count, 0, output),
        "dwindle_mod"=> window_tree=basic::dwindle(window_tree,client_count, 1, output),
        "left"=> window_tree=layout::master_stack(window_tree,output, client_count, master_count, master_factor),
        "center"=> window_tree=layout::centered_master(window_tree,output, client_count, master_count, master_factor),
        "magic"=> window_tree=layout::magic_master(window_tree,output, client_count, master_count, master_factor),
        "nested"=> window_tree=layout::nested(window_tree,output, client_count, master_count, master_factor),
        _ => {
            println!("{} isn't a valid layout", layout);
            std::process::exit(0);
        }
    }

    window_tree
}


pub fn combi(layouts:Vec<&str>,mut window_tree:Vec<basic::Frame>, output:basic::Frame, mut client_count:u32, master_count:u32, master_factor:f32) -> Vec<basic::Frame> {

    let main_tree:Vec<basic::Frame>=Vec::new();
    let default_client_count:u32=client_count/layouts.len() as u32;

    let main_frame:Vec<basic::Frame>=if client_count <= layouts.len() as u32 {
        basic::vertical(main_tree.clone(), client_count, 1, master_factor, output)
    } else {
        basic::vertical(main_tree.clone(), layouts.len() as u32, 1, master_factor, output)
    };

    for i in 0..main_frame.len() {
        let chosen_count:u32;
        if i == 0 && layouts.len() > 1 {
            chosen_count=master_count;
            window_tree=chosen_layout(layouts[i],window_tree, main_frame[i], chosen_count, 1, master_factor);
        } else {
            chosen_count= if i < main_frame.len()-1 {
                default_client_count
            } else { client_count };
            window_tree=chosen_layout(layouts[i],window_tree, main_frame[i], chosen_count, 1, master_factor);
        }

        client_count-=chosen_count;
    }

    window_tree
}
