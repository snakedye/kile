use crate::base::Frame;

pub fn tab(mut frame_tree:Vec<Frame>, frame:Frame, client_count:u32) -> Vec<Frame> {

    let mut window:Frame=frame;

    for _i in 0..client_count {
        frame_tree.push(window);
        if client_count > 1 {
            window.h-=30;
            window.y+=30;
        }
    }

    frame_tree
}

pub fn horizontal(mut frame_tree:Vec<Frame>, output:Frame, client_count:u32, main_count:u32, main_factor:f32) -> Vec<Frame> {

    let mut frame:Frame=output;

    let master_height:u32= if main_count > 0 {
        output.h*((main_factor * 100.0) as u32)/(50*client_count)
    } else { 0 };

    let slave_height:u32= if main_count > 0 && client_count > 1 {
        (output.h-master_height)/(client_count-1)
    } else if main_count < 1 && client_count > 0 {
        (output.h-master_height)/(client_count)
    } else { output.h };

    for i in 0..client_count {
        
        if client_count > 1 {
            frame.h= if i+1==main_count {
                master_height
            } else if i < client_count-1 {
                slave_height
            } else {
                output.y+output.h-frame.y
            }
        }
        frame_tree.push(frame);
        frame.y+=frame.h;
   }

    frame_tree

}

pub fn vertical(mut frame_tree:Vec<Frame>, output:Frame, client_count:u32, main_count:u32, main_factor:f32) -> Vec<Frame> {

    let mut frame:Frame=output;

    let master_width:u32= if main_count > 0 {
        output.w*((main_factor * 100.0) as u32)/(50*client_count)
    } else { 0 };

    let slave_width:u32= if main_count > 0 && client_count > 1 {
        (output.w-master_width)/(client_count-1)
    } else if main_count < 1 && client_count > 0 {
        (output.w-master_width)/(client_count)
    } else { output.w };

    for i in 0..client_count {
        
        if client_count > 1 {
            frame.w= if i+1==main_count {
                master_width
            } else if i < client_count-1 {
                slave_width
            } else {
                output.x+output.w-frame.x
            }
        }
        frame_tree.push(frame);
        frame.x+=frame.w;
   }

    frame_tree

}

pub fn dwindle(mut frame_tree:Vec<Frame>, output:Frame, client_count:u32, modi:u32) -> Vec<Frame> {

    let mut frame:Frame=output;

    for i in 0..client_count {
        let mut index=frame_tree.len();
        if i > 0 && index > 0 {
            index-=1;
            if (i+modi)%2!=0 {
                frame.h/=2;
                frame_tree[(index) as usize].h-=frame.h;
                frame.y+=frame.h;
            } else {
                frame.w/=2;
                frame_tree[index as usize].w-=frame.w;
                frame.x+=frame.w;
            }
        }

        frame_tree.push(frame);
    }

    frame_tree
}

