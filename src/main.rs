use std::env;
mod basic;
mod layout;

fn main() {

    let args: Vec<String> = env::args().collect(); // arguments vector

    if args.len() < 7 {
        println!("Not enough arguments");
        std::process::exit(0);
    }

    let layout:&str=&args[1];
    let client_count:u32=args[2].parse().unwrap();
    let master_count:u32=args[3].parse().unwrap();
    let master_width_factor:f32=args[4].parse().unwrap();
    let screen_width:u32=args[5].parse().unwrap();
    let screen_height:u32=args[6].parse().unwrap();

    let mut window_tree:Vec<basic::Frame>=Vec::new();

    let output:basic::Frame=basic::new_frame(0,0,screen_width,screen_height);

    match layout {
        "left" => window_tree=layout::master_stack(window_tree,output, client_count, master_count, master_width_factor),
        "center" => window_tree=layout::centered_master(window_tree,output, client_count, master_count, master_width_factor),
        "magic" => window_tree=layout::magic_master(window_tree,output, client_count, master_count, master_width_factor),
        "nested" => window_tree=layout::nested(window_tree,output, client_count, master_count, master_width_factor),
        _ => {
            println!("{} isn't a valid layout", layout);
            std::process::exit(0);
        }
    }

    for window in window_tree {
        println!("{} {} {} {}", window.x, window.y, window.w, window.h);
    }
}

