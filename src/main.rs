use std::env;

#[derive(Copy, Clone)]
#[derive(Debug)]

struct Window{
    x: u32,
    y: u32,
    width: u32,
    height: u32
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let client_count:u32=args[1].parse().unwrap();
    let master_count:u32=args[2].parse().unwrap();
    let master_width_factor:f32=args[3].parse().unwrap();
    let screen_width:u32=args[4].parse().unwrap();
    let screen_height:u32=args[5].parse().unwrap();

    let mut node_tree:Vec<Window>=Vec::new();

    node_tree=stack(node_tree, client_count, master_count, master_width_factor, screen_width, screen_height);

    for window in node_tree {
        println!("{} {} {} {}", window.x, window.y, window.width, window.height);
    }
}

fn stack(mut node_tree:Vec<Window>, client_count:u32, master_count:u32, master_width_factor:f32, screen_width:u32, screen_height:u32) -> Vec<Window> {
    let master_width: u32=screen_width*((master_width_factor * 10.0) as u32)/10;
    let master=Window{
        x: 0,
        y: 0,
        width: master_width,
        height: screen_height,
    };

    let slave_width: u32=screen_width-master_width;
    let slave=Window{
        x: master_width,
        y: 0,
        width: slave_width,
        height: screen_height,
    };

    let fullscreen=Window{
        x: 0,
        y: 0,
        width: screen_width,
        height: screen_height,
    };

    for i in 0..client_count {
        if client_count==1 {
            node_tree.push(fullscreen);
            break;
        }
        if i != master_count-1 {
            node_tree.push(slave);
        } else {
            node_tree.push(master);
        }
    }

    node_tree
}
