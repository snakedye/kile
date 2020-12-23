// Layouts for river

#[derive(Copy, Clone)]

pub struct Window{
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32
}

pub fn stack(mut node_tree:Vec<Window>, client_count:u32, master_count:u32, master_width_factor:f32, screen_width:u32, screen_height:u32) -> Vec<Window> {

    let master_width: u32=screen_width*((master_width_factor * 100.0) as u32)/100;

    let master=Window{
        x: 0,
        y: 0,
        width: master_width,
        height: screen_height,
    };

    let slave=Window{
        x: master_width,
        y: 0,
        width: screen_width-master_width,
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
