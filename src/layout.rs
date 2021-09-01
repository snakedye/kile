use super::client::*;

#[derive(Copy, Clone, Debug)]
pub enum Condition {
    Equal,
    Greater,
    Less,
}

impl Condition {
    fn is_true(&self, limit: u32, amount: u32) -> bool {
        match self {
            Condition::Equal => limit == amount,
            Condition::Greater => amount > limit,
            Condition::Less => amount < limit,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Layout {
    Full,
    Deck,
    Vertical,
    Horizontal,
    Recursive {
        outer: Box<Layout>,
        inner: Vec<Layout>,
    },
    Conditional {
        amount: u32,
        a: Box<Layout>,
        b: Box<Layout>,
        condition: Condition,
    },
    Parameters {
        layout: Box<Layout>,
        amount: u32,
        index: u32,
        factor: f64,
    },
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
        mut view_amount: u32,
        layout: &Layout,
        list: &mut Vec<Area>,
        parent: bool,
        factor: bool,
    ) {
        let mut area = self;
        let master = parent && factor && view_amount > 1 && parameters.main_index < view_amount;

        match layout {
            Layout::Full => {
                for _i in 0..view_amount {
                    list.push(area);
                }
            }
            Layout::Deck => {
                let yoffset = ((self.h as f64 * 0.1) / (view_amount as f64 - 1.0)).floor() as u32;
                let xoffset = ((self.w as f64 * 0.1) / (view_amount as f64 - 1.0)).floor() as u32;
                for _i in 0..view_amount {
                    area.w = self.w - (xoffset * (view_amount - 1));
                    area.h = self.h - (yoffset * (view_amount - 1));
                    list.push(area);
                    area.x += xoffset;
                    area.y += yoffset;
                }
            }
            Layout::Horizontal => {
                let reste = area.h % view_amount;
                let mut slave_height = area.h;
                let main_height = if master {
                    ((area.h as f64) * parameters.main_factor) as u32
                } else {
                    0
                };
                slave_height -= main_height;
                for i in 0..view_amount {
                    area.h = if master && i == parameters.main_index {
                        main_height
                    } else if master {
                        slave_height / (view_amount - 1)
                    } else {
                        slave_height / view_amount
                    };
                    if i == 0 {
                        area.h += reste;
                    }

                    list.push(area);
                    area.y += area.h;
                }
            }
            Layout::Vertical => {
                let reste = area.w % view_amount;
                let mut slave_width = area.w;
                let main_width = if master {
                    ((area.w as f64) * parameters.main_factor) as u32
                } else {
                    0
                };
                slave_width -= main_width;
                for i in 0..view_amount {
                    area.w = if master && i == parameters.main_index {
                        main_width
                    } else if master {
                        slave_width / (view_amount - 1)
                    } else {
                        slave_width / view_amount
                    };
                    if i == 0 {
                        area.w += reste;
                    }

                    list.push(area);
                    area.x += area.w;
                }
            }
            Layout::Recursive { outer, inner } => {
                let mut frame = Vec::new();
                let frames_available = inner.len() as u32;
                let mut frame_amount = {
                    let main = parameters.main_amount >= 1
                        && frames_available > 1
                        && parameters.main_index < frames_available
                        && view_amount > parameters.main_amount;
                    if parameters.main_amount >= view_amount {
                        1
                    } else if main && view_amount - parameters.main_amount < frames_available {
                        1 + view_amount - parameters.main_amount
                    } else if view_amount > frames_available || main {
                        frames_available
                    } else {
                        view_amount
                    }
                };
                area.generate(parameters, frame_amount, &*outer, &mut frame, true, factor);
                if parent
                    && parameters.main_amount > 0
                    && parameters.main_amount <= view_amount
                    && parameters.main_index < frame_amount
                {
                    frame_amount -= 1;
                    view_amount -= parameters.main_amount;
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
                    let mut count = view_amount / frame_amount;
                    if view_amount % frame_amount != 0 && i as u32 != frame_amount {
                        view_amount -= 1;
                        count += 1;
                    }
                    if frame_amount as usize != inner.len() && i >= parameters.main_index as usize {
                        i += 1
                    }
                    rect.generate(parameters, count, &inner[i], list, false, false)
                }
            }
            Layout::Parameters {
                layout,
                amount,
                index,
                factor,
            } => {
                let parameters = {
                    Parameters {
                        main_amount: *amount,
                        main_index: *index,
                        main_factor: *factor,
                    }
                };
                area.generate(&parameters, view_amount, &*layout, list, true, true);
            }
            Layout::Conditional {
                a,
                b,
                amount,
                condition,
            } => {
                if condition.is_true(*amount, view_amount) {
                    area.generate(&parameters, view_amount, &*a, list, true, true);
                } else {
                    area.generate(&parameters, view_amount, &*b, list, true, true);
                }
            }
        }
    }
}
