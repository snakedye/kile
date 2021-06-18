use crate::layout::*;

#[derive(Copy, Clone, Debug)]
enum Expression<'s> {
    Some(&'s str),
    None,
}

#[derive(Copy, Clone, Debug)]
struct Tape<'s> {
    current: Expression<'s>,
    next: Expression<'s>,
}

impl<'s> Tape<'s> {
    fn new(current: Expression<'s>, next: Expression<'s>) -> Tape<'s> {
        Tape {
            current: current,
            next: next,
        }
    }
}

impl<'s> Expression<'s> {
    fn new(string: &'s str) -> Expression {
        Expression::Some(string)
    }
    fn len(&self) -> usize {
        match self {
            Expression::Some(s) => s.len(),
            Expression::None => 0,
        }
    }
    // Splits an expression in 2 at the index of a character
    fn split_ounce(self, pattern: char) -> Tape<'s> {
        let (mut paren, mut brace) = (0, 0);
        if let Expression::Some(s) = self {
            for (i, c) in s.to_string().chars().enumerate() {
                match c {
                    '(' => paren += 1,
                    ')' => paren -= 1,
                    '{' => brace += 1,
                    '}' => brace -= 1,
                    _ => {}
                }
                if c == pattern && brace == 0 && paren == 0 {
                    return Tape::new(
                        Expression::Some(&s[0..i].trim()),
                        if &s[i + 1..] != "" {
                            Expression::new(&s[i + 1..].trim())
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
                        return Expression::new(&s[start + 1..i].trim());
                    }
                }
            }
        }
        Expression::None
    }
    // Iterates over all expressions delimited by a character
    // and excutes a function on each one of them
    fn filter(&self, pattern: char, mut f: impl FnMut(Expression) -> Result<(), &'static str>) {
        let tape = self.split_ounce(pattern);
        match f(tape.current) {
            Ok(_) => {
                if let Expression::Some(_) = tape.next {
                    tape.next.filter(pattern, f);
                }
            }
            Err(m) => {
                if m != "" {
                    println!("{}", m)
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
                    '{' => {
                        // Turns "{ a : b }" into this: current: a, next: b
                        let tape = Expression::new(name).clamp('{', '}').split_ounce(':');

                        Layout::Recursive {
                            outer: { Box::new(layout(tape.current.release())) },
                            inner: {
                                let mut vec = Vec::new();
                                tape.next.filter(' ', |s| {
                                    if s.len() == s.clamp('{', '}').len() + 2
                                        || s.len() == s.clamp('(', ')').len() + 2
                                    {
                                        return Err("");
                                    }
                                    vec.push(layout(s.release()));
                                    Ok(())
                                });
                                vec
                            },
                        }
                    }
                    '(' => {
                        let mut i = 0;
                        let mut var: (u32, f64, u32) = (0, 0.6, 0);
                        let tape = Expression::new(name).clamp('(', ')').split_ounce(' ');
                        // Dispatches layout values to the field corresponding to an index
                        tape.next.filter(' ', |s| {
                            i += 1;
                            match i {
                                1 => match s.release().parse::<u32>() {
                                    Ok(main_amount) => {
                                        var.0 = main_amount;
                                    }
                                    Err(_) => return Err("Invalid main amount"),
                                },
                                2 => match s.release().parse::<f64>() {
                                    Ok(main_factor) => {
                                        var.1 = main_factor;
                                    }
                                    Err(_) => return Err("Invalid main factor"),
                                },
                                3 => match s.release().parse::<u32>() {
                                    Ok(main_index) => {
                                        var.2 = main_index;
                                    }
                                    Err(_) => return Err("Invalid main index"),
                                },
                                _ => {}
                            }
                            Ok(())
                        });

                        Layout::Assisted {
                            layout: Box::new(layout(tape.current.release())),
                            amount: var.0,
                            factor: var.1,
                            index: var.2,
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
