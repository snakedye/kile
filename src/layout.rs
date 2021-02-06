use crate::basic;

pub fn master_stack(mut window_tree:Vec<basic::Frame>, output:basic::Frame, client_count:u32, master_count:u32, master_factor:f32) -> Vec<basic::Frame> {

    let main_tree:Vec<basic::Frame>=Vec::new();

    match client_count {
        1=> {
            let main_frame:Vec<basic::Frame>=basic::vertical(main_tree, 1, 1, master_factor, output);
            window_tree=basic::tab(window_tree, 1, main_frame[0]);
        }
        _=> {
            let main_frame:Vec<basic::Frame>=basic::vertical(main_tree, 2, 1, master_factor, output);
            window_tree=basic::horizontal(window_tree, master_count, 0, master_factor, main_frame[0]);
            window_tree=basic::horizontal(window_tree, client_count-master_count, 0, master_factor, main_frame[1]);
        }
    }

    window_tree
}

pub fn centered_master(mut window_tree:Vec<basic::Frame>, output:basic::Frame, client_count:u32, master_count:u32, master_factor:f32) -> Vec<basic::Frame> {

    let main_tree:Vec<basic::Frame>=Vec::new();

    match client_count {
        1=> {
            let main_frame:Vec<basic::Frame>=basic::vertical(main_tree, 1, 1, master_factor, output);
            window_tree=basic::tab(window_tree, 1, main_frame[0]);
        }
        2=> {
            let main_frame:Vec<basic::Frame>=basic::vertical(main_tree, 2, 1, master_factor, output);
            window_tree=basic::tab(window_tree, 1, main_frame[0]);
            window_tree=basic::tab(window_tree, 1, main_frame[1]);
        }
        _=> {
            let main_frame:Vec<basic::Frame>=basic::vertical(main_tree, 3, 2, master_factor, output);
            window_tree=basic::tab(window_tree, master_count, main_frame[1]);
            window_tree=basic::horizontal(window_tree, (client_count-master_count)/2, 0, master_factor, main_frame[0]);
            window_tree=basic::dwindle(window_tree, client_count-master_count-((client_count-master_count)/2), 0, main_frame[2]);
        }
    };

    window_tree
}

pub fn magic_master(mut window_tree:Vec<basic::Frame>, output:basic::Frame, client_count:u32, master_count:u32, master_factor:f32) -> Vec<basic::Frame> {

    let main_tree:Vec<basic::Frame>=Vec::new();

    match client_count {
        1=> {
            let main_frame:Vec<basic::Frame>=basic::vertical(main_tree, 1, 1, master_factor, output);
            window_tree=basic::tab(window_tree, 1, main_frame[0]);
        }
        _=> {
            let main_frame:Vec<basic::Frame>=basic::vertical(main_tree, 2, 1, master_factor, output);
            window_tree=centered_master(window_tree, main_frame[0], master_count, 1, 0.5);
            window_tree=master_stack(window_tree, main_frame[1], client_count-master_count, 1, 0.5);
        }
    }

    window_tree
}

pub fn nested(mut window_tree:Vec<basic::Frame>, output:basic::Frame, client_count:u32, master_count:u32, master_factor:f32) -> Vec<basic::Frame> {

    let main_tree:Vec<basic::Frame>=Vec::new();

    match client_count {
        1 => {
            let main_frame:Vec<basic::Frame>=basic::vertical(main_tree, 1, 1, master_factor, output);
            window_tree=basic::tab(window_tree, 1, main_frame[0]);
        }
        2 => {
            let main_frame:Vec<basic::Frame>=basic::vertical(main_tree, 2, 1, master_factor, output);
            window_tree=basic::tab(window_tree, 1, main_frame[0]);
            window_tree=basic::tab(window_tree, 1, main_frame[1]);
        }
        _=> {
            let main_frame:Vec<basic::Frame>=basic::dwindle(main_tree, 3, 1, output);
            window_tree=basic::tab(window_tree, master_count, main_frame[0]);
            window_tree=basic::dwindle(window_tree, (client_count-master_count)/2, 1, main_frame[1]);
            window_tree=basic::vertical(window_tree, client_count-master_count-((client_count-master_count)/2), 0, master_factor, main_frame[2]);
        }
    };

    window_tree
}
