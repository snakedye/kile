use super::client::*;

#[derive(Copy, Clone, Debug)]
pub enum Layout {
    Tab,
    Full,
    Vertical,
    Recursive { modi: u32 },
    Horizontal,
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
    pub fn compare(&self, rule: &Rule) -> bool {
        match self {
            Rectangle::Window(window) => window.compare(rule),
            _ => false,
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
        options: &mut Options,
        client_count: u32,
        layout: Layout,
        parent: bool,
        factor: bool,
    ) -> Vec<Rectangle> {
        let mut area = self.area();
        let mut list = Vec::new();
        let view_amount = options.windows.len() as u32;

        match layout {
            Layout::Tab => {
                for _i in 0..client_count {
                    insert_window(&mut list, area, options, parent);
                    area.h -= 50;
                    area.y += 50;
                }
            }
            Layout::Horizontal => {
                let mut main_area = area;
                let mut slave_area = area;
                let reste = area.h % client_count;
                if factor {
                    main_area.h = if options.main_amount > 0
                        && client_count > 1
                        && options.main_amount < view_amount
                        && options.main_index < client_count
                    {
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

                    insert_window(&mut list, area, options, parent);
                    area.y += area.h;
                }
            }
            Layout::Vertical => {
                let mut main_area = area;
                let mut slave_area = area;
                let reste = area.w % client_count;
                if factor {
                    main_area.w = if options.main_amount > 0
                        && client_count > 1
                        && options.main_amount < view_amount
                        && options.main_index < client_count
                    {
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

                    insert_window(&mut list, area, options, parent);
                    area.x += area.w;
                }
            }
            Layout::Recursive { modi } => {
                let master = if factor {
                    if options.main_amount > 0
                        && client_count > 1
                        && options.main_amount < view_amount
                        && options.main_index < client_count
                    {
                        true
                    } else {
                        false
                    }
                } else {
                    false
                };
                for i in 0..client_count {
                    let mut reste;
                    if i < client_count - 1 {
                        if (i + modi) % 2 == 0 {
                            if master && i == options.main_index {
                                area.w = ((area.w as f64) * options.main_factor) as u32;
                            } else {
                                reste = area.w % 2;
                                area.w /= 2;
                                area.w += reste;
                            }
                            insert_window(&mut list, area, options, parent);
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
                            insert_window(&mut list, area, options, parent);
                            area.y += area.h;
                            if master && i == options.main_index {
                                area.h = (((area.h as f64) * (1.0 - options.main_factor))
                                    / options.main_factor)
                                    .ceil() as u32;
                            }
                        }
                    } else {
                        insert_window(&mut list, area, options, parent);
                    }
                }
            }
            Layout::Full => {
                for _i in 0..client_count {
                    insert_window(&mut list, area, options, parent);
                }
            }
        }
        list
    }
}

fn insert_window(list: &mut Vec<Rectangle>, area: Area, options: &mut Options, parent: bool) {
    if !parent && options.windows.len() > 0 {
        let mut window = options.windows.remove(0);
        window.area = Some(area);
        list.push(Rectangle::Window(window));
    } else {
        list.push(Rectangle::Area(area));
    };
}
