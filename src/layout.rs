use super::client::*;
use crate::wayland::river_layout_v2::river_layout_v2::RiverLayoutV2;
use wayland_client::Main;

#[derive(Clone, Debug)]
pub enum Layout {
    Tab,
    Full,
    Vertical,
    Dwindle ( u32 ),
    Recursive { outer: Box<Layout>, inner: Vec<Layout> },
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
    options: Options,
    parent: bool,
) {
    if !parent {
        area.apply_padding(options.view_padding);
        layout.push_view_dimensions(serial, area.x as i32, area.y as i32, area.w, area.h);
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
        mut options: Options,
        client_count: u32,
        layout: &Layout,
        list: &mut Vec<Rectangle>,
        parent: bool,
        factor: bool,
    ) {
        let mut area = self.area();

        let master = if parent
            && factor
            && client_count > 1
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
                if master {
                    main_area.h = 
                        ((area.h as f64) * options.main_factor * 0.5 * (client_count as f64)) as u32;
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

                    insert_window(serial, zlayout, list, area, options, parent);
                    area.y += area.h;
                }
            }
            Layout::Vertical => {
                let mut main_area = area;
                let mut slave_area = area;
                let reste = area.w % client_count;
                if master {
                    main_area.w = 
                        ((area.w as f64) * options.main_factor * 0.5 * (client_count as f64)) as u32;
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
                    let main = options.main_amount >= 1
                        && frames_available > 1
                        && options.main_index < frames_available
                        && client_count > options.main_amount;
                    if main
                        && client_count - options.main_amount < frames_available {
                        1 + client_count - options.main_amount
                    } else if client_count > frames_available
                        || main {
                        frames_available
                    } else if options.main_amount >= client_count {
                        1
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
                        options.main_amount = client_count - options.main_index
                    }
                } else {
                    options.main_amount = 0
                };
                use std::ops::Deref;
                Rectangle::Area(area).generate(
                    serial,
                    zlayout,
                    options,
                    frame_amount,
                    outer.deref(),
                    &mut frame,
                    true,
                    factor,
                );

                let mut reste = if options.main_amount > 0 {
                    slave_amount = (client_count - options.main_amount) / (frame_amount - 1);
                    (client_count - options.main_amount) % (frame_amount - 1)
                } else {
                    slave_amount = client_count / frame_amount;
                    client_count % frame_amount
                };

                if options.main_amount != 0 {
                    frame[options.main_index as usize].generate(
                        serial,
                        zlayout,
                        options,
                        options.main_amount,
                        &inner[options.main_index as usize],
                        list,
                        false,
                        false,
                    );
                }

                for (i, rect) in frame.iter_mut().enumerate() {
                    if options.main_amount != 0
                        && i == options.main_index as usize { continue }
                    rect.generate(
                        serial,
                        zlayout,
                        options,
                        if reste > 0 {
                            reste -= 1;
                            slave_amount + 1
                        } else {
                            slave_amount
                        },
                        &inner[i],
                        list,
                        false,
                        false,
                    )
                }
            }
            Layout::Full => {
                for _i in 0..client_count {
                    insert_window(serial, zlayout, list, area, options, parent);
                }
            }
        }
    }
}
