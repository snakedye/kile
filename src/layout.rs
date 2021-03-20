use crate::display::{Frame, State, Tag};
use crate::options::{Layout, Options};

pub fn new(
    mut frame: Frame,
    options: &Options,
    layout: (Layout, u32, State),
    frames: &mut Vec<Frame>,
) {
    let (layout, client_count, state) = layout;

    if client_count > 0 {
        match layout {
            Layout::Tab => {
                // Add eww titlebar eventually
                for _i in 0..client_count {
                    frames.push(frame);
                    frame.h -= 30;
                    frame.y += 30;
                    frame.y -= options.view_padding;
                }
            }
            Layout::Horizontal => {
                let mut is_main = 0;
                let mut slave_area = frame;
                let mut main_area = frame;
                let reste = frame.h % client_count;
                if state == State::Frame {
                    main_area.h = if options.main_count > 1 {
                        (frame.h * (options.main_factor * 100.0) as u32) / (50 * client_count)
                    } else {
                        0
                    };
                    slave_area.h -= main_area.h;
                    if options.main_index < options.view_amount && options.main_count > 0 {
                        is_main = 1;
                    }
                }
                for i in 0..client_count {
                    if state == State::Frame && i == options.main_index && options.main_count > 0 {
                        frame.h = main_area.h;
                    } else {
                        frame.h = slave_area.h / client_count;
                    }
                    frame.h -= options.view_padding;
                    if i == 0 {
                        frame.h += reste;
                    }

                    frames.push(frame);
                    frame.y += frame.h + options.view_padding;
                }
            }
            Layout::Vertical => {
                let mut is_main = 0;
                let mut slave_area = frame;
                let mut main_area = frame;
                let reste = frame.w % client_count;
                if state == State::Frame {
                    main_area.w = if options.main_count > 1 {
                        (frame.w * (options.main_factor * 100.0) as u32) / (50 * client_count)
                    } else {
                        0
                    };
                    slave_area.w -= main_area.w;
                    if options.main_index < options.view_amount && options.main_count > 0 {
                        is_main = 1;
                    }
                }
                for i in 0..client_count {
                    if state == State::Frame && i == options.main_index && options.main_count > 0 {
                        frame.w = main_area.w;
                    } else {
                        frame.w = slave_area.w / (client_count - is_main);
                    }
                    frame.w -= options.view_padding;
                    if i == 0 {
                        frame.w += reste;
                    }

                    frames.push(frame);
                    frame.x += frame.w + options.view_padding;
                }
            }
            Layout::Full => {
                for _i in 0..client_count {
                    frame.y -= options.view_padding;
                    frames.push(frame);
                }
            }
        }
    }
}
