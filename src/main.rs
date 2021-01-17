use std::env;
mod layout;

fn main() {

    let args: Vec<String> = env::args().collect(); // arguments vector

    if args.len() != 7 {
        println!("Not enough arguments");
        std::process::exit(0);
    }

    let layout:&str=&args[1];
    let client_count:u32=args[2].parse().unwrap();
    let master_count:u32=args[3].parse().unwrap();
    let master_width_factor:f32=args[4].parse().unwrap();
    let screen_width:u32=args[5].parse().unwrap();
    let screen_height:u32=args[6].parse().unwrap();

    let mut node_tree:Vec<layout::Window>=Vec::new();

    // The default layout is stack

    match layout {
        "tab" => node_tree=layout::stack(node_tree, client_count, master_count, master_width_factor, screen_width, screen_height),
        "hive" => node_tree=layout::hive(node_tree, client_count, master_count, master_width_factor, screen_width, screen_height),
        "grid" => node_tree=layout::grid(node_tree, client_count, master_count, master_width_factor, screen_width, screen_height),
        "left" => node_tree=layout::left(node_tree, client_count, master_count, master_width_factor, screen_width, screen_height),
        "dwindle" => node_tree=layout::dwindle(node_tree, client_count, master_count, master_width_factor, screen_width, screen_height),
        _ => {
            println!("{} isn't a valid layout", layout);
            std::process::exit(0);
        }
    }

    for window in node_tree {
        println!("{} {} {} {}", window.x, window.y, window.width, window.height);
    }
}

