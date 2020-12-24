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

    if client_count<2 {
        let fullscreen=Window{
            x: 0,
            y: 0,
            width: screen_width,
            height: screen_height,
        };

        node_tree.push(fullscreen);
        return node_tree;
    }

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

    for i in 1..client_count+1 {
        if i == master_count {
            node_tree.push(master);
        }
        else if (master_count < 1 || master_count > client_count) && i == 1 {
            node_tree.push(master);
        }
        else {
            node_tree.push(slave);
        }
    }

    node_tree
}

pub fn hive(mut node_tree:Vec<Window>, client_count:u32, master_count:u32, master_width_factor:f32, screen_width:u32, screen_height:u32) -> Vec<Window> {

    let master_width: u32=screen_width*((master_width_factor * 100.0) as u32)/150;

    let branch_count: u32=(client_count-1)/2;

    if client_count<3 {
        node_tree=stack(node_tree, client_count, master_count, master_width_factor, screen_width, screen_height);
        return node_tree
    } else {

        let mut slave=Window{
            x: 0,
            y: 0,
            width: (screen_width-master_width)/2,
            height: screen_height/branch_count,
        };

        let master=Window{
            x: (screen_width-master_width)/2,
            y: 0,
            width: master_width,
            height: screen_height,
        };

        let mut left_count:u32=0;
        let mut right_count:u32=0;

        for i in 1..client_count+1 {
            if i==master_count {
                node_tree.push(master);
            }
            else if (master_count < 1 || master_count > client_count) && i == 1 {
                node_tree.push(master);
            }
            else {
                if left_count<branch_count {
                    slave.x=0;
                    slave.height=screen_height/branch_count;
                    slave.y=left_count*slave.height;

                    node_tree.push(slave);
                    left_count+=1;
                }
                else {
                    slave.x=master_width+((screen_width-master_width)/2);
                    slave.height=screen_height/(client_count-branch_count-1);
                    slave.y=right_count*slave.height;

                    node_tree.push(slave);
                    right_count+=1;
                }
            }
        }
    }

    node_tree

}

