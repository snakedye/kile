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

    let mut slave=Window{
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
            slave.height-=30;
            slave.y+=30;
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

pub fn grid(mut node_tree:Vec<Window>, client_count:u32, master_count:u32, master_width_factor:f32, screen_width:u32, screen_height:u32) -> Vec<Window> {

    let views:u32=(client_count as f32).sqrt().ceil() as u32;

    let master_view:u32= if views*views != client_count {
        1
    } else { 0 };

    let mut master_view_width:u32=screen_width*((master_width_factor * 100.0) as u32)/100;

    if 50 > master_view_width || master_view_width > screen_width-50 {
        master_view_width=screen_width/views;
    }

    let slave_view_width:u32=(screen_width-master_view_width)/(views-1);

    let mut window=Window{
        x:0,
        y:0,
        width: slave_view_width,
        height:0,
    };

    let clients:u32=client_count-((views-master_view)*(views-master_view));

    for i in 0..views {

        window.x=window.width*i;

        if i < views-master_view {
            for w in 0..views-master_view {
                if master_view==0 && i==views-1 {
                    window.width=master_view_width;
                }
                window.height=screen_height/(views-master_view);
                window.y=window.height*w;
                node_tree.push(window);
            }

        } else {

            for w in 0..clients {
                window.width=master_view_width;
                window.height=screen_height/clients;
                window.y=window.height*w;
                node_tree.push(window);
            }

        }

    }

    node_tree

}
