use crate::layout::*;

#[derive(Copy, Clone, Debug)]
pub enum Expression<'s> {
    Some(&'s str),
    None,
}

#[derive(Copy, Clone, Debug)]
pub struct Tape<'s> {
    current: Expression<'s>,
    next: Expression<'s>,
}

impl<'s> Tape<'s> {
    pub fn new(current: Expression<'s>, next: Expression<'s>) -> Tape<'s> {
        Tape { current, next }
    }
    pub fn drop(self) -> (&'s str, &'s str) {
        (self.current.release(), self.next.release())
    }
}

impl<'s> Expression<'s> {
    pub fn new(string: &'s str) -> Expression {
        Expression::Some(string)
    }
    fn is_some(&self) -> bool {
        match self {
            Expression::None => false,
            _ => true,
        }
    }
    // Splits an expression in 2 at the index of a character
    pub fn split_ounce(self, pattern: char) -> Tape<'s> {
        let mut paren = 0;
        if let Expression::Some(s) = self {
            for (i, c) in s.to_string().chars().enumerate() {
                match c {
                    '(' => paren += 1,
                    ')' => paren -= 1,
                    _ => {}
                }
                if c == pattern && paren == 0 {
                    return Tape::new(
                        Expression::Some(s[0..i].trim()),
                        if !&s[i + 1..].is_empty() {
                            Expression::new(s[i + 1..].trim())
                        } else {
                            Expression::None
                        },
                    );
                }
            }
        }
        Tape::new(self, Expression::None)
    }
    fn split_for(self, mut f: impl FnMut(char) -> bool) -> Tape<'s> {
        if let Expression::Some(s) = self {
            for (i, c) in s.to_string().chars().enumerate() {
                if f(c) {
                    return Tape::new(
                        Expression::Some(s[0..i].trim()),
                        if !&s[i + 1..].is_empty() {
                            Expression::new(s[i + 1..].trim())
                        } else {
                            Expression::None
                        },
                    );
                }
            }
        }
        Tape::new(self, Expression::None)
    }
    // Captures a string slice contained within specific patterns
    fn clamp(&self, opening: char, closing: char) -> Expression<'s> {
        let (mut start, mut brace) = (0, 0);
        if let Expression::Some(s) = self {
            for (i, c) in s.to_string().chars().enumerate() {
                if c == opening {
                    if brace == 0 {
                        start = i;
                    }
                    brace += 1;
                } else if c == closing {
                    brace -= 1;
                    if brace == 0 {
                        return Expression::new(s[start + 1..i].trim());
                    }
                }
            }
        }
        Expression::None
    }
    // Iterates over all expressions delimited by a character
    // and excutes a function on each one of them
    fn filter(&self, pattern: char, mut f: impl FnMut(Expression) -> Result<(), String>) {
        let tape = self.split_ounce(pattern);
        match f(tape.current) {
            Ok(_) => {
                if let Expression::Some(_) = tape.next {
                    tape.next.filter(pattern, f);
                }
            }
            Err(m) => {
                if !m.is_empty() {
                    eprintln!("{}", m)
                }
            }
        }
    }
    fn release(self) -> &'s str {
        if let Expression::Some(s) = self {
            s
        } else {
            ""
        }
    }
}

pub fn layout<'s>(name: &str) -> Layout {
    match name {
        "f" | "ful" | "full" => Layout::Full,
        "d" | "dec" | "deck" => Layout::Deck,
        "v" | "ver" | "vertical" => Layout::Vertical,
        "h" | "hor" | "horizontal" => Layout::Horizontal,
        _ => {
            if let Some(char) = name.chars().next() {
                match char {
                    '(' => {
                        let mut condition = None;
                        let exp = Expression::new(name).clamp('(', ')');
                        let mut tape = exp.split_ounce(':');
                        if tape.next.is_some() {
                            Layout::Recursive {
                                outer: { Box::new(layout(tape.current.release())) },
                                inner: {
                                    let mut vec = Vec::new();
                                    tape.next.filter(' ', |s| {
                                        vec.push(layout(s.release()));
                                        Ok(())
                                    });
                                    vec
                                },
                            }
                        } else if {
                            let mut paren = 0;
                            tape = exp.split_for(|c| {
                                match c {
                                    '(' => paren += 1,
                                    ')' => paren -= 1,
                                    '>' => {
                                        if paren == 0 {
                                            condition = Some(Condition::Greater);
                                            return true;
                                        }
                                    }
                                    '=' => {
                                        if paren == 0 {
                                            condition = Some(Condition::Equal);
                                            return true;
                                        }
                                    }
                                    '<' => {
                                        if paren == 0 {
                                            condition = Some(Condition::Less);
                                            return true;
                                        }
                                    }
                                    _ => {}
                                }
                                false
                            });
                            tape.next.is_some()
                        } {
                            let variant = tape.current.release();
                            if let Ok(uint) = variant.parse::<u32>() {
                                let mut layouts = tape.next.split_ounce('?');
                                if layouts.next.is_some() {
                                    Layout::Conditional {
                                        variant: Variant::Amount(uint),
                                        condition: condition.unwrap(),
                                        a: Box::new(layout(layouts.current.release())),
                                        b: Box::new(layout(layouts.next.release())),
                                    }
                                } else if {
                                    layouts = tape.next.split_ounce('!');
                                    layouts.next.is_some()
                                } {
                                    Layout::Conditional {
                                        variant: Variant::Index(uint),
                                        condition: condition.unwrap(),
                                        a: Box::new(layout(layouts.current.release())),
                                        b: Box::new(layout(layouts.next.release())),
                                    }
                                } else {
                                    Layout::Full
                                }
                            } else if let Ok(float) = variant.parse::<f64>() {
                                let (a, b) = tape.next.split_ounce('%').drop();
                                Layout::Conditional {
                                    variant: Variant::Ratio(float),
                                    condition: condition.unwrap(),
                                    a: Box::new(layout(a)),
                                    b: Box::new(layout(b)),
                                }
                            } else {
                                Layout::Full
                            }
                        } else {
                            let mut i = 0;
                            let tape = exp.split_ounce(' ');
                            let mut var: (u32, f64, u32) = (0, 0.6, 0);
                            // Dispatches layout values to the field corresponding to an index
                            tape.next.filter(' ', |s| {
                                i += 1;
                                match i {
                                    1 => match s.release().parse::<u32>() {
                                        Ok(main_amount) => {
                                            var.0 = main_amount;
                                        }
                                        Err(e) => {
                                            return Err(format!("Invalid main amount: {}", e))
                                        }
                                    },
                                    2 => match s.release().parse::<f64>() {
                                        Ok(main_ratio) => {
                                            var.1 = main_ratio;
                                        }
                                        Err(e) => return Err(format!("Invalid main ratio: {}", e)),
                                    },
                                    3 => match s.release().parse::<u32>() {
                                        Ok(main_index) => {
                                            var.2 = main_index;
                                        }
                                        Err(e) => return Err(format!("Invalid main index: {}", e)),
                                    },
                                    _ => {}
                                }
                                Ok(())
                            });

                            Layout::Parameters {
                                layout: Box::new(layout(tape.current.release())),
                                amount: var.0,
                                ratio: var.1,
                                index: var.2,
                            }
                        }
                    }
                    _ => Layout::Full,
                }
            } else {
                Layout::Full
            }
        }
    }
}
