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
        parameters: &Parameters,
        mut client_count: u32,
        layout: &Layout,
        list: &mut Vec<Area>,
        parent: bool,
        factor: bool,
    ) {
        let mut area = self;
        let master = if parent && factor && client_count > 1 && parameters.main_index < client_count {
            true
        } else {
            false
        };

        match layout {
            Layout::Tab => while client_count > 0 {
                let delta = parameters.main_factor * 100.0;
                list.push(area);
                area.h -= delta as u32;
                area.y += delta as u32;
                client_count -= 1;
            }
            Layout::Horizontal => {
                let reste = area.h % client_count;
                let mut slave_height = area.h;
                let main_height = if master {
                    ((area.h as f64) * parameters.main_factor) as u32
                } else {
                    0
                };
                slave_height -= main_height;
                for i in 0..client_count {
                    area.h = if master && i == parameters.main_index {
                        main_height
                    } else if master {
                        slave_height / (client_count - 1)
                    } else {
                        slave_height / client_count
                    };
                    if i == 0 {
                        area.h += reste;
                    }

                    list.push(area);
                    area.y += area.h;
                }
            }
            Layout::Vertical => {
                let reste = area.w % client_count;
                let mut slave_width = area.w;
                let main_width = if master {
                    ((area.w as f64) * parameters.main_factor) as u32
                } else {
                    0
                };
                slave_width -= main_width;
                for i in 0..client_count {
                    area.w = if master && i == parameters.main_index {
                        main_width
                    } else if master {
                        slave_width / (client_count - 1)
                    } else {
                        slave_width / client_count
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
                            if master && i == parameters.main_index {
                                area.w = ((area.w as f64) * parameters.main_factor) as u32;
                            } else {
                                reste = area.w % 2;
                                area.w /= 2;
                                area.w += reste;
                            }
                            list.push(area);
                            area.x += area.w;
                            if master && i == parameters.main_index {
                                area.w = (((area.w as f64) * (1.0 - parameters.main_factor))
                                    / parameters.main_factor)
                                    .ceil() as u32;
                            }
                        } else {
                            if master && i == parameters.main_index {
                                area.h = ((area.h as f64) * parameters.main_factor) as u32;
                            } else {
                                reste = area.h % 2;
                                area.h /= 2;
                                area.h += reste;
                            }
                            list.push(area);
                            area.y += area.h;
                            if master && i == parameters.main_index {
                                area.h = (((area.h as f64) * (1.0 - parameters.main_factor))
                                    / parameters.main_factor)
                                    .ceil() as u32;
                            }
                        }
                    } else {
                        list.push(area);
                    }
                }
            }
            Layout::Recursive { outer, inner } => {
                let mut frame = Vec::new();
                let frames_available = inner.len() as u32;
                let mut frame_amount = {
                    let main = parameters.main_amount >= 1
                        && frames_available > 1
                        && parameters.main_index < frames_available
                        && client_count > parameters.main_amount;
                    if parameters.main_amount >= client_count {
                        1
                    } else if main && client_count - parameters.main_amount < frames_available {
                        1 + client_count - parameters.main_amount
                    } else if client_count > frames_available || main {
                        frames_available
                    } else {
                        client_count
                    }
                };
                area.generate( parameters, frame_amount, outer.deref(), &mut frame, true, factor,);
                if parent && parameters.main_amount > 0
                    && parameters.main_amount < client_count
                    && parameters.main_index < frame.len() as u32 {
                    frame_amount -= 1;
                    client_count -= parameters.main_amount;
                    frame.remove(parameters.main_index as usize).generate(
                        parameters,
                        parameters.main_amount,
                        &inner[parameters.main_index as usize],
                        list,
                        false,
                        false,
                    );
                }
                for (mut i, rect) in frame.iter_mut().enumerate() {
                    let mut count = client_count / frame_amount;
                    if client_count % frame_amount != 0 && i as u32 != frame_amount {
                        client_count -= 1;
                        count += 1;
                    }
                    if parent 
                        && parameters.main_amount > 0 && i >= parameters.main_index as usize { i+=1 }
                    rect.generate(parameters, count, &inner[i], list, false, false)
                }
            }
            Layout::Full => while client_count > 0 {
                list.push(area);
                client_count -= 1;
            }
        }
    }
}
