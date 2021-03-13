use crate::frame::Frame;
use crate::Option::Layout;
use crate::custom::dwindle;

pub fn layout(view:&mut child, options:Options) {

    let capacity=view.capacity;
    let client_count=options.client_count;
    let main_index=options.main_index;
    let main_count=options.main_count;
    let main_factor=options.main_factor;

    let tree=view.get_tree();

    let mut child=view.clone();

    match layout {
        Layout::Tab => {
            // Add eww titlebar eventually
            let i=0;
            while ( i < capacity && i < client_count )
                view.push(child);
                if client_count > 1 {
                    child.h-=30;
                    child.y+=30;
                }
                options.client_count-=1;
                i++;
            }
        }
        Layout::Horizontal => {
            let i=0;
            let ajusted=true;
            while ( view.parent.state==State::Output, i < capacity && i < client_count ) {

                if i==main_index && main_count>0 {
                    child.set_main();
                    child.h=view.h*((main_factor * 100.0) as u32)/(50*main_count)-view.padding;
                } else {
                    child.set_slave();
                    child.h=view.h*(((1.0-main_factor) * 100.0) as u32)/(50*(client_count-main_count))-view.padding;
                }

                if ! ajusted && i!=main {
                    child.h+=view.h%child.h;
                }

                view.push();
                child.y+=child.h+view.padding;
                options.client_count-=1;
                i+=1;
           }
        }
        Layout::Vertical => {
            let i=0;
            let ajusted=true;
            while ( i < capacity && i < client_count ) {

                if i==main_index && main_count>0 {
                    child.set_main();
                    child.w=view.w*((main_factor * 100.0) as u32)/(50*main_count)-view.padding;
                } else {
                    child.set_slave();
                    child.w=view.w*(((1.0-main_factor) * 100.0) as u32)/(50*(client_count-main_count))-view.padding;
                }

                if ! ajusted && i!=main {
                    child.w+=view.w%child.w;
                }

                view.push();
                child.h+=child.w+view.padding;
                i+=1;
            }
        }
        Layout::Full => {
            let i=0;
            while ( i < capacity && i < client_count ) {
                view.push(child);
                options.client_count-=1;
                i+=1;
            }
        }
    }

    if client_count>capacity {
        view.callback(options);
    }
}
