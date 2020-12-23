use std::env;
mod layout;

fn main() {

    let args: Vec<String> = env::args().collect(); // arguments vector

    let client_count:u32=args[1].parse().unwrap();
    let master_count:u32=args[2].parse().unwrap();
    let master_width_factor:f32=args[3].parse().unwrap();
    let screen_width:u32=args[4].parse().unwrap();
    let screen_height:u32=args[5].parse().unwrap();

    let mut node_tree:Vec<layout::Window>=Vec::new();

    node_tree=layout::stack(node_tree, client_count, master_count, master_width_factor, screen_width, screen_height);

    for window in node_tree {
        println!("{} {} {} {}", window.x, window.y, window.width, window.height);
    }
}

