use std::env;
mod basic;
mod layout;
mod combi;

fn main() {

    let args: Vec<String> = env::args().collect(); // arguments vector
    let mut layouts: Vec<&str> = Vec::new(); // layout vector

    let mut client_count:u32=0;
    let mut master_count:u32=0;
    let mut master_width_factor:f32=0.1;
    let mut screen_width:u32=0;
    let mut screen_height:u32=0;

    for i in 1..args.len() {
        if i <= args.len()-6 {
            // println!("layout: {}", args[i]);
            layouts.push(&args[i]);
        } else {
            let index=i-layouts.len()-1;
            match index {
                0usize=>client_count=args[i].parse().unwrap(),
                1usize=>master_count=args[i].parse().unwrap(),
                2usize=>master_width_factor=args[i].parse().unwrap(),
                3usize=>screen_width=args[i].parse().unwrap(),
                4usize=>screen_height=args[i].parse().unwrap(),
                _ => {
                    println!("Too much arguments");
                    std::process::exit(0);
                }
            }
        }
    }

    let mut window_tree:Vec<basic::Frame>=Vec::new();

    let output:basic::Frame=basic::new_frame(0,0,screen_width,screen_height);

    if layouts.len() > 1 {
        window_tree=combi::combi(layouts, window_tree, output, client_count, master_count, master_width_factor);
    } else {
        window_tree=combi::chosen_layout(layouts[0], window_tree, output, client_count, master_count, master_width_factor);
    }

    for window in window_tree {
        println!("{} {} {} {}", window.x, window.y, window.w, window.h);
    }
}
