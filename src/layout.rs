use crate::display::Tag;
use crate::options::{Layout, Options};

pub fn engine(tag: &mut Tag, options: &Options) {
    let client_count = if tag.client_count > options.view_amount {
        tag.client_count - options.view_amount
    } else {
        options.view_amount
    };
    let main_index = options.main_index;
    let main_count = if options.main_index+options.main_count>client_count {
        client_count-options.main_index
    } else { options.main_count };
    let main_factor = options.main_factor;

    let layout_vec=options.parse();

    let mut frame = tag.main_frame.clone();

    let mut i = 0;
    let mut ajusted = false;
    while i < client_count {
        if options.view_amount > 0 {frame.apply_padding(&options.view_padding);}
        if i >= main_index && i < main_index+main_index && main_count > 0 {
           frame.set_main();
        } else { frame.set_slave() }
        match layout_vec[i as usize] {
            Layout::Tab => {
                // Add eww titlebar eventually
                if i > 1 {
                    frame.h -= 30;
                    frame.y += 30;
                }
            }
            Layout::Horizontal => {
                if frame.is_main() {
                    frame.h *= (main_factor * 100.0) as u32 / (50 * main_count);
                } else {
                    frame.h /= 2;
                }

                if !ajusted && i != main_index {
                    frame.h += tag.main_frame.h % frame.h;
                    ajusted = true;
                }

                frame.y += frame.h;
            }
            Layout::Vertical => {
                if frame.is_main() {
                    frame.w *= ((main_factor * 100.0) as u32) / (50 * main_count);
                } else {
                    frame.w *= (((1.0 - main_factor) * 100.0) as u32)
                        / (100 * 2);
                }

                if !ajusted && i != main_index {
                    frame.w += tag.main_frame.w % frame.w;
                    ajusted = true;
                }

                frame.h += frame.w;
            }
            Layout::Full => { }
        }
        i += 1;
        tag.windows.push(frame);
        options.push_dimensions(&frame);
    }
    // Send commit message to the compositor
    options.commit();
}
