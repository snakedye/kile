mod base;
mod layout;
mod custom;

use std::env;
use crate::base::Frame;
use crate::base::Layout;
use crate::base::State;

fn main() {

    let args: Vec<String> = env::args().collect();
    let mut layouts: Vec<&str> = Vec::new();
    let mut client_count:u32=0;
    let mut main_count:u32=0;
    let mut main_factor:f32=0.0;
    let mut width:u32=0;
    let mut height:u32=0;

    for i in 1..args.len() {
        if i <= args.len()-6 {
            layouts.push(&args[i]);
        } else {
            let index=i-layouts.len();
            match index {
                1usize=>client_count=args[i].parse().unwrap(),
                2usize=>{
                    let arg:u32=args[i].parse().unwrap();
                    main_count= if arg < 1 {
                        1
                    } else if arg > client_count {
                        client_count
                    } else { arg };
                }
                3usize=>{
                    let arg:f32=args[i].parse().unwrap();
                    main_factor= if arg < 0.0 {
                        0.0
                    } else if arg > 1.0 {
                        1.0
                    } else { arg };
                }
                4usize=>width=args[i].parse().unwrap(),
                5usize=>height=args[i].parse().unwrap(),
                _ => {},
            }
        }
    }


    let mut window_tree:Vec<Frame>=Vec::new();

    let mut output:Frame=Frame::new(0,0,width,height,client_count,main_count,0,main_factor,State::Slave,Layout::Full);

    output.validate();

    if layouts.len() > 1 {
        custom::combi(&mut window_tree, layouts, output);
    } else {
        output.set_layout(layouts[0]);
        output.generate(&mut window_tree);
    }

    for window in window_tree {
        println!("{} {} {} {}", window.x, window.y, window.w, window.h);
    }
}

