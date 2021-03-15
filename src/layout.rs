use crate::display::{Tag};
use crate::options::{Options, Layout};

pub fn engine(tag:&mut Tag, options:&Options){

    let client_count=if tag.client_count > options.view_amount {
        tag.client_count-options.view_amount
    } else {
        options.view_amount
    };
    let main_index=options.main_index;
    let main_count=options.main_count;
    let main_factor=options.main_factor;

    let mut frame=tag.main_frame.clone();

    let mut i=0;
    let mut ajusted=false;
    while i < client_count {
        match options.arguments[i as usize] {
            Layout::Tab => {
                // Add eww titlebar eventually
                options.push_dimensions(&frame);
                if client_count > 1 {
                    frame.h-=30;
                    frame.y+=30;
                }
                tag.windows.push(frame);
            }
            Layout::Horizontal => {
                if i==main_index && main_count>0 {
                    frame.set_main();
                    frame.h=tag.main_frame.h*((main_factor * 100.0) as u32)/(50*main_count)-options.view_padding;
                } else {
                    frame.set_slave();
                    frame.h=tag.main_frame.h*(((1.0-main_factor) * 100.0) as u32)/(50*(client_count-main_count))-options.view_padding;
                }

                if ! ajusted && i!=main_index {
                    frame.h+=tag.main_frame.h%frame.h;
                    ajusted=true;
                }

                tag.windows.push(frame);
                options.push_dimensions(&frame);
                frame.y+=frame.h+options.view_padding;
            }
            Layout::Vertical => {
                if i==main_index && main_count>0 {
                    frame.set_main();
                    frame.w=tag.main_frame.w*((main_factor * 100.0) as u32)/(50*main_count)-options.view_padding;
                } else {
                    frame.set_slave();
                    frame.w=tag.main_frame.w*(((1.0-main_factor) * 100.0) as u32)/(50*(client_count-main_count))-options.view_padding;
                }

                if ! ajusted && i!=main_index {
                    frame.w+=tag.main_frame.w%frame.w;
                    ajusted=true;
                }

                tag.windows.push(frame);
                options.push_dimensions(&frame);
                frame.h+=frame.w+options.view_padding;
            }
            Layout::Full => {
                tag.windows.push(frame);
            }
        }
        i+=1;
    }
    // Send commit message to the compositor
    options.commit();
}
