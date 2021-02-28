mod frame;
mod layout;
mod custom;

use std::env;
use crate::frame::Frame;

fn main() {

    let args: Vec<String> = env::args().collect();
    let mut layouts: Vec<&str> = Vec::new();
    let mut output:Frame=Frame::new();

    for i in 1..args.len() {
        if i <= args.len()-7 {
            layouts.push(&args[i]);
        } else {
            let index=i-layouts.len();
            match index {
                2usize=>output.set_client_count(args[i].parse::<u32>().unwrap()),
                3usize=>{
                    output.set_main_count(args[i].parse::<u32>().unwrap());
                    let main_index=args[i-2].parse::<i32>().unwrap();
                    if main_index > 0 {output.set_main_index(main_index as u32)}
                }
                4usize=>output.set_main_factor(args[i].parse::<f32>().unwrap()),
                5usize=>output.w=args[i].parse::<u32>().unwrap(),
                6usize=>output.h=args[i].parse::<u32>().unwrap(),
                _ => {},
            }
        }
    }

    output.fix();
    output.validate();

    let mut window_tree:Vec<Frame>=Vec::new();
    if layouts.len() > 1 {
        custom::combi::generate(&mut window_tree, layouts, output);
    } else {
        output.set_layout(layouts[0]);
        output.generate(&mut window_tree);
    }

    for window in window_tree {
        println!("{} {} {} {}", window.x, window.y, window.w, window.h);
    }
}

