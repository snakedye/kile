use super::client::*;
use std::ops::Deref;

#[derive(Clone, Debug)]
pub enum Layout {
    Tab,
    Full,
    Vertical,
    Dwindle(u32),
    Recursive {
        outer: Box<Layout>,
        inner: Vec<Layout>,
    },
    Horizontal,
}

impl Area {
    pub fn new(x: u32, y: u32, w: u32, h: u32) -> Area {
        {
            Area {
                x: x,
                y: y,
                w: w,
                h: h,
            }
        }
    }
    pub fn apply_padding(&mut self, padding: i32) {
        if 2 * padding < self.h as i32 && 2 * padding < self.w as i32 {
            self.x = ((self.x as i32) + padding) as u32;
            self.y = ((self.y as i32) + padding) as u32;
            self.w = ((self.w as i32) - 2 * padding) as u32;
            self.h = ((self.h as i32) - 2 * padding) as u32;
        }
    }
    pub fn generate(
        self,
        mut options: Options,
        mut client_count: u32,
        layout: &Layout,
        list: &mut Vec<Area>,
        parent: bool,
        factor: bool,
    ) {
        let mut area = self;

        let master = if parent && factor && client_count > 1 && options.main_index < client_count {
            true
        } else {
            false
        };
        match layout {
            Layout::Tab => {
                while client_count > 0 {
                    list.push(area);
                    area.h -= 50;
                    area.y += 50;
                    client_count-=1;
                }
            }
            Layout::Horizontal => {
                let mut main_area = area;
                let mut slave_area = area;
                let reste = area.h % client_count;
                if master {
                    main_area.h = ((area.h as f64) * options.main_factor) as u32;
                } else {
                    main_area.h = 0;
                }
                slave_area.h -= main_area.h;
                for i in 0..client_count {
                    area.h = if factor && i == options.main_index && main_area.h > 0 {
                        main_area.h
                    } else if factor && main_area.h > 0 {
                        slave_area.h / (client_count - 1)
                    } else {
                        slave_area.h / client_count
                    };
                    if i == 0 {
                        area.h += reste;
                    }

                    list.push(area);
                    area.y += area.h;
                }
            }
            Layout::Vertical => {
                let mut main_area = area;
                let mut slave_area = area;
                let reste = area.w % client_count;
                if master {
                    main_area.w = ((area.w as f64) * options.main_factor) as u32;
                } else {
                    main_area.w = 0;
                }
                slave_area.w -= main_area.w;
                for i in 0..client_count {
                    area.w = if factor && i == options.main_index && main_area.w > 0 {
                        main_area.w
                    } else if factor && main_area.w > 0 {
                        slave_area.w / (client_count - 1)
                    } else {
                        slave_area.w / client_count
                    };
                    if i == 0 {
                        area.w += reste;
                    }

                    list.push(area);
                    area.x += area.w;
                }
            }
            Layout::Dwindle(modi) => {
                for i in 0..client_count {
                    let reste;
                    if i < client_count - 1 {
                        if (i + modi) % 2 == 0 {
                            if master && i == options.main_index {
                                area.w = ((area.w as f64) * options.main_factor) as u32;
                            } else {
                                reste = area.w % 2;
                                area.w /= 2;
                                area.w += reste;
                            }
                            list.push(area);
                            area.x += area.w;
                            if master && i == options.main_index {
                                area.w = (((area.w as f64) * (1.0 - options.main_factor))
                                    / options.main_factor)
                                    .ceil() as u32;
                            }
                        } else {
                            if master && i == options.main_index {
                                area.h = ((area.h as f64) * options.main_factor) as u32;
                            } else {
                                reste = area.h % 2;
                                area.h /= 2;
                                area.h += reste;
                            }
                            list.push(area);
                            area.y += area.h;
                            if master && i == options.main_index {
                                area.h = (((area.h as f64) * (1.0 - options.main_factor))
                                    / options.main_factor)
                                    .ceil() as u32;
                            }
                        }
                    } else {
                        list.push(area);
                    }
                }
            }
            Layout::Recursive { outer, inner } => {
                let mut main_amount = 0;
                let mut frame = Vec::new();
                let frames_available = inner.len() as u32;
                let mut frame_amount = {
                    let main = options.main_amount >= 1
                        && frames_available > 1
                        && options.main_index < frames_available
                        && client_count > options.main_amount;
                    if options.main_amount >= client_count {
                        1
                    } else if main && client_count - options.main_amount < frames_available {
                        1 + client_count - options.main_amount
                    } else if client_count > frames_available || main {
                        frames_available
                    } else {
                        client_count
                    }
                };
                if parent
                    && frame_amount > 1
                    && options.main_amount > 0
                    && options.main_index < frames_available
                {
                    if options.main_index + options.main_amount > client_count {
                        main_amount = client_count - options.main_index;
                    } else {
                        main_amount = options.main_amount;
                    }
                }
                area.generate(
                    options,
                    frame_amount,
                    outer.deref(),
                    &mut frame,
                    true,
                    factor,
                );

                if main_amount != 0 {
                    frame_amount -= 1;
                    options.main_amount = 0;
                    client_count -= main_amount;
                    frame[options.main_index as usize].generate(
                        options,
                        main_amount,
                        &inner[options.main_index as usize],
                        list,
                        false,
                        false,
                    );
                }

                for (i, rect) in frame.iter_mut().enumerate() {
                    if main_amount != 0 && i == options.main_index as usize {
                        continue;
                    }
                    let mut count = client_count / frame_amount;
                    if client_count % frame_amount != 0 && i as u32 != frame_amount {
                        client_count -= 1;
                        count += 1;
                    }
                    rect.generate(options, count, &inner[i], list, false, false)
                }
            }
            Layout::Full => {
                while client_count > 0 {
                    list.push(area);
                    client_count -= 1;
                }
            }
        }
    }
}
