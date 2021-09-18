use super::client::*;

#[derive(Copy, Clone, Debug)]
pub enum Condition {
    Equal,
    Greater,
    Less,
}

#[derive(Copy, Clone, Debug)]
pub enum Variant {
    Amount(u32),
    Index(u32),
    Ratio(f64),
}

impl Condition {
    fn is_true(&self, variant: &Variant, parameters: Parameters) -> bool {
        match self {
            Condition::Equal => match *variant {
                Variant::Amount(uint) => parameters.amount == uint,
                Variant::Index(uint) => parameters.index == uint,
                Variant::Ratio(float) => parameters.ratio == float,
            },
            Condition::Greater => match *variant {
                Variant::Amount(uint) => parameters.amount > uint,
                Variant::Index(uint) => parameters.index > uint,
                Variant::Ratio(float) => parameters.ratio > float,
            },
            Condition::Less => match *variant {
                Variant::Amount(uint) => parameters.amount < uint,
                Variant::Index(uint) => parameters.index < uint,
                Variant::Ratio(float) => parameters.ratio < float,
            },
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
        a: Box<Layout>,
        b: Box<Layout>,
        variant: Variant,
        condition: Condition,
    },
    Parameters {
        index: Option<u32>,
        ratio: Option<f64>,
        amount: Option<u32>,
        layout: Box<Layout>,
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
        views: &mut Vec<Area>,
        layout: &Layout,
        parameters: &Parameters,
        mut view_amount: u32,
        ratio: bool,
    ) {
        let mut area = self;
        let master = ratio && view_amount > 1 && parameters.index < view_amount;

        match layout {
            Layout::Full => {
                for _ in 0..view_amount {
                    views.push(area);
                }
            }
            Layout::Deck => {
                let yoffset = ((self.h as f64 * 0.1) / (view_amount as f64 - 1.0)).floor() as u32;
                let xoffset = ((self.w as f64 * 0.1) / (view_amount as f64 - 1.0)).floor() as u32;
                for _ in 0..view_amount {
                    area.w = self.w - (xoffset * (view_amount - 1));
                    area.h = self.h - (yoffset * (view_amount - 1));
                    views.push(area);
                    area.x += xoffset;
                    area.y += yoffset;
                }
            }
            Layout::Horizontal => {
                let reste = area.h % view_amount;
                let mut slave_height = area.h;
                let main_height = if master {
                    ((area.h as f64) * parameters.ratio) as u32
                } else {
                    0
                };
                slave_height -= main_height;
                for i in 0..view_amount {
                    area.h = if master && i == parameters.index {
                        main_height
                    } else if master {
                        slave_height / (view_amount - 1)
                    } else {
                        slave_height / view_amount
                    };
                    if i == 0 {
                        area.h += reste;
                    }

                    views.push(area);
                    area.y += area.h;
                }
            }
            Layout::Vertical => {
                let reste = area.w % view_amount;
                let mut slave_width = area.w;
                let main_width = if master {
                    ((area.w as f64) * parameters.ratio) as u32
                } else {
                    0
                };
                slave_width -= main_width;
                for i in 0..view_amount {
                    area.w = if master && i == parameters.index {
                        main_width
                    } else if master {
                        slave_width / (view_amount - 1)
                    } else {
                        slave_width / view_amount
                    };
                    if i == 0 {
                        area.w += reste;
                    }

                    views.push(area);
                    area.x += area.w;
                }
            }
            Layout::Recursive { outer, inner } => {
                let mut frame = Vec::new();
                let frames_available = inner.len() as u32;
                let mut frame_amount = {
                    let main = parameters.amount >= 1
                        && frames_available > 1
                        && parameters.index < frames_available
                        && view_amount > parameters.amount;
                    if parameters.amount >= view_amount {
                        1
                    } else if main && view_amount - parameters.amount < frames_available {
                        1 + view_amount - parameters.amount
                    } else if view_amount > frames_available || main {
                        frames_available
                    } else {
                        view_amount
                    }
                };
                area.generate(&mut frame, &*outer, parameters, frame_amount, ratio);
                if parameters.amount > 0
                    && parameters.amount <= view_amount
                    && parameters.index < frame_amount
                {
                    frame_amount -= 1;
                    frame.remove(parameters.index as usize).generate(
                        views,
                        &inner[parameters.index as usize],
                        parameters,
                        if frame_amount == 0 {
                            view_amount
                        } else { parameters.amount },
                        false,
                    );
                    view_amount -= parameters.amount;
                }
                for (mut i, rect) in frame.iter_mut().enumerate() {
                    let mut count = view_amount / frame_amount;
                    if view_amount % frame_amount != 0 && i as u32 != frame_amount {
                        view_amount -= 1;
                        count += 1;
                    }
                    if frame_amount as usize != inner.len() && i >= parameters.index as usize {
                        i += 1
                    }
                    rect.generate(views, &inner[i], parameters, count, false);
                }
            }
            Layout::Parameters {
                index,
                ratio,
                amount,
                layout,
            } => {
                let parameters = {
                    Parameters {
                        index: index.unwrap_or(parameters.index),
                        ratio: ratio.unwrap_or(parameters.ratio),
                        amount: amount.unwrap_or(parameters.amount),
                    }
                };
                area.generate(views, &*layout, &parameters, view_amount, true);
            }
            Layout::Conditional {
                a,
                b,
                variant,
                condition,
            } => {
                if condition.is_true(
                    variant,
                    Parameters {
                        amount: view_amount,
                        ratio: parameters.ratio,
                        index: parameters.index,
                    },
                ) {
                    area.generate(views, &*a, &parameters, view_amount, ratio);
                } else {
                    area.generate(views, &*b, &parameters, view_amount, ratio);
                }
            }
        }
    }
}
