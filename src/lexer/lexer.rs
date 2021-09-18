use crate::layout::*;

pub fn split_ounce<'s>(exp: &'s str, pattern: char) -> Option<(&'s str, &'s str)> {
    let mut paren = 0;
    for (i, c) in exp.to_string().chars().enumerate() {
        match c {
            '(' => paren += 1,
            ')' => paren -= 1,
            _ => {}
        }
        if c == pattern && paren == 0 {
            if !&exp[i + 1..].is_empty() {
                return Some((exp[0..i].trim(), exp[i + 1..].trim()));
            }
        }
    }
    None
}

// Captures a string slice contained within specific patterns
fn clamp<'s>(exp: &'s str) -> Option<&'s str> {
    if let Some(start) = exp.find('(') {
        if let Some(end) = exp.rfind(')') {
            return Some(&exp[start + 1..end].trim());
        }
    }
    None
}
// Iterates over all expressions delimited by a character
// and excutes a function on each one of them
fn filter<'s>(exp: &'s str, pattern: char, mut f: impl FnMut(&'s str) -> Result<(), String>) {
    if let Some((head, tail)) = split_ounce(exp, pattern) {
        match f(head) {
            Ok(_) => {
                filter(tail, pattern, f);
            }
            Err(m) => {
                if !m.is_empty() {
                    eprintln!("{}", m)
                }
            }
        }
    } else {
        match f(exp) {
            Err(m) => {
                if !m.is_empty() {
                    eprintln!("{}", m)
                }
            }
            _ => {}
        }
    }
}

pub fn parse<'s>(name: &str) -> Layout {
    match name {
        "f" | "ful" | "full" => Layout::Full,
        "d" | "dec" | "deck" => Layout::Deck,
        "v" | "ver" | "vertical" => Layout::Vertical,
        "h" | "hor" | "horizontal" => Layout::Horizontal,
        _ => {
            let mut condition = None;
            let (mut value, mut layout) = (None, None);
            if let Some(exp) = clamp(name) {
                if let Some((outer, inner)) = split_ounce(exp, ':') {
                    Layout::Recursive {
                        outer: { Box::new(parse(outer)) },
                        inner: if !inner.is_empty() {
                            let mut vec = Vec::new();
                            filter(inner, ' ', |s| {
                                vec.push(parse(s));
                                Ok(())
                            });
                            vec
                        } else {
                            eprintln!("Unsufficient amount of sublayouts: {}", exp);
                            vec![Layout::Full]
                        },
                    }
                } else if {
                    if let Some((v, l)) = split_ounce(exp, '>') {
                        value = Some(v);
                        layout = Some(l);
                        condition = Some(Condition::Greater);
                    } else if let Some((v, l)) = split_ounce(exp, '<') {
                        value = Some(v);
                        layout = Some(l);
                        condition = Some(Condition::Less);
                    } else if let Some((v, l)) = split_ounce(exp, '=') {
                        value = Some(v);
                        layout = Some(l);
                        condition = Some(Condition::Equal);
                    }
                    condition.is_some()
                } {
                    if let Ok(uint) = value.unwrap().parse::<u32>() {
                        if let Some((a, b)) = split_ounce(layout.unwrap(), '?') {
                            Layout::Conditional {
                                variant: Variant::Amount(uint),
                                condition: condition.unwrap(),
                                a: Box::new(parse(a)),
                                b: Box::new(parse(b)),
                            }
                        } else if let Some((a, b)) = split_ounce(layout.unwrap(), '!') {
                            Layout::Conditional {
                                variant: Variant::Index(uint),
                                condition: condition.unwrap(),
                                a: Box::new(parse(a)),
                                b: Box::new(parse(b)),
                            }
                        } else {
                            Layout::Full
                        }
                    } else if let Ok(float) = value.unwrap().parse::<f64>() {
                        if let Some((a, b)) = split_ounce(layout.unwrap(), '%') {
                            Layout::Conditional {
                                variant: Variant::Ratio(float),
                                condition: condition.unwrap(),
                                a: Box::new(parse(a)),
                                b: Box::new(parse(b)),
                            }
                        } else {
                            Layout::Full
                        }
                    } else {
                        Layout::Full
                    }
                } else {
                    if let Some((layout, parameters)) = split_ounce(exp, ' ') {
                        let mut var = parameters.split_whitespace();
                        Layout::Parameters {
                            layout: Box::new(parse(layout)),
                            amount: if let Ok(uint) = var.next().unwrap_or_default().parse::<u32>()
                            {
                                Some(uint)
                            } else {
                                None
                            },
                            ratio: if let Ok(float) = var.next().unwrap_or_default().parse::<f64>()
                            {
                                Some(float)
                            } else {
                                None
                            },
                            index: if let Ok(uint) = var.next().unwrap_or_default().parse::<u32>() {
                                Some(uint)
                            } else {
                                None
                            },
                        }
                    } else {
                        Layout::Full
                    }
                }
            } else {
                Layout::Full
            }
        }
    }
}
