use super::client::*;
use crate::wayland::river_layout_v2::river_layout_v2::RiverLayoutV2;
use wayland_client::Main;

#[derive(Clone, Debug)]
pub enum Layout {
    Tab,
    Full,
    Vertical,
    Dwindle ( u32 ),
    Recursive { outer: Box<Option<Layout>>, inner: Vec<Layout> },
    Horizontal,
}

fn zoom(list: &mut Vec<Rectangle>, index: usize) {
    if (index as usize) < list.len() {
        let area = list[index].area();
        for i in (0..index).rev() {
            let previous = list[i].area();
            list[i + 1].set(previous);
        }
        list[0].set(area);
    }
}

fn insert_window(
    serial: u32,
    layout: &Main<RiverLayoutV2>,
    list: &mut Vec<Rectangle>,
    mut area: Area,
    options: &mut Options,
    parent: bool,
) {
    if !parent && options.windows.len() > 0 {
        let mut window = options.windows.remove(0);
        area.apply_padding(options.view_padding);
        window.area = if window.compare(&options.rule) {
            layout.push_view_dimensions(serial, area.x as i32, area.y as i32, area.w, area.h);
            Some(list.remove(0).area())
        } else {
            Some(area)
        };
        list.push(Rectangle::Window(window));
    } else {
        list.push(Rectangle::Area(area));
    };
}

impl Rectangle {
    pub fn apply_padding(&mut self, padding: u32) {
        match self {
            Rectangle::Window(window) => window.apply_padding(padding),
            Rectangle::Area(area) => area.apply_padding(padding),
        }
    }
    pub fn area(&self) -> Area {
        match self {
            Rectangle::Window(window) => window.area.unwrap(),
            Rectangle::Area(area) => *area,
        }
    }
    pub fn set(&mut self, area: Area) {
        match self {
            Rectangle::Window(window) => window.area = Some(area),
            Rectangle::Area(area_handle) => (*area_handle) = area,
        }
    }
    pub fn generate(
        &mut self,
        serial: u32,
        zlayout: &Main<RiverLayoutV2>,
        options: &mut Options,
        client_count: u32,
        layout: &Layout,
        list: &mut Vec<Rectangle>,
        parent: bool,
        factor: bool,
    ) {
        let mut area = self.area();
        let view_amount = options.windows.len() as u32;

        let master = if factor
            && options.main_amount > 0
            && client_count > 1
            && options.main_amount < view_amount
            && options.main_index < client_count
        {
            true
        } else {
            false
        };
        match layout {
            Layout::Tab => {
                for _i in 0..client_count {
                    insert_window(serial, zlayout, list, area, options, parent);
                    area.h -= 50;
                    area.y += 50;
                }
            }
            Layout::Horizontal => {
                let mut main_area = area;
                let mut slave_area = area;
                let reste = area.h % client_count;
                if factor {
                    main_area.h = if master {
                        (area.h * (options.main_factor * 100.0) as u32) / (50 * client_count)
                    } else {
                        0
                    };
                    slave_area.h -= main_area.h;
                }
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

                    insert_window(serial, zlayout, list, area, options, parent);
                    area.y += area.h;
                }
            }
            Layout::Vertical => {
                let mut main_area = area;
                let mut slave_area = area;
                let reste = area.w % client_count;
                if factor {
                    main_area.w = if master {
                        (area.w * (options.main_factor * 100.0) as u32) / (50 * client_count)
                    } else {
                        0
                    };
                    slave_area.w -= main_area.w;
                }
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

                    insert_window(serial, zlayout, list, area, options, parent);
                    area.x += area.w;
                }
            }
            Layout::Dwindle ( modi ) => {
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
                            insert_window(serial, zlayout, list, area, options, parent);
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
                            insert_window(serial, zlayout, list, area, options, parent);
                            area.y += area.h;
                            if master && i == options.main_index {
                                area.h = (((area.h as f64) * (1.0 - options.main_factor))
                                    / options.main_factor)
                                    .ceil() as u32;
                            }
                        }
                    } else {
                        insert_window(serial, zlayout, list, area, options, parent);
                    }
                }
            }
            Layout::Recursive{ inner, outer } => {
                let slave_amount;
                let mut frame = Vec::new();
                let frames_available = inner.len() as u32;
                let frame_amount = {
                    let main = options.main_amount > 1
                        && frames_available > 1
                        && client_count > options.main_amount;
                    if main {
                        if client_count - options.main_amount < frames_available {
                            1 + client_count - options.main_amount
                        } else {
                            frames_available
                        }
                    } else if options.main_amount >= client_count {
                        1
                    } else if client_count > frames_available {
                        frames_available
                    } else {
                        client_count
                    }
                };
                let main_amount = if parent
                    && options.main_index + options.main_amount <= client_count
                    && frame_amount > 1
                    && options.main_amount > 0
                {
                    if options.main_index + options.main_amount > client_count {
                        client_count - options.main_index
                    } else {
                        options.main_amount
                    }
                } else {
                    0
                };
                use std::ops::Deref;
                Rectangle::Area(area).generate(
                    serial,
                    zlayout,
                    options,
                    frame_amount,
                    outer.deref().as_ref().unwrap(),
                    &mut frame,
                    true,
                    factor,
                );

                let mut reste = if main_amount > 0 {
                    zoom(&mut frame, options.main_index as usize);
                    slave_amount = (client_count - main_amount) / (frame_amount - 1);
                    (client_count - main_amount) % (frame_amount - 1)
                } else {
                    slave_amount = client_count / frame_amount;
                    client_count % frame_amount
                };

                for (i, rect) in frame.iter_mut().enumerate() {
                    let amount = if i == 0 && main_amount != 0 {
                        main_amount
                    } else {
                        if reste > 0 {
                            reste -= 1;
                            slave_amount + 1
                        } else {
                            slave_amount
                        }
                    };
                    rect.generate(
                        serial,
                        zlayout,
                        options,
                        amount,
                        &inner[i],
                        list,
                        false,
                        false,
                    )
                }
            }
            Layout::Full => {
                for i in 0..client_count {
                    insert_window(serial, zlayout, list, area, options, parent);
                }
            }
        }
    }
}
