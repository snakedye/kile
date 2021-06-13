use crate::layout::*;

#[derive(Copy, Clone, Debug)]
enum Match<'s> {
    Some(&'s str),
    None,
}

#[derive(Copy, Clone, Debug)]
struct Tape<'s> {
    current: Match<'s>,
    next: Match<'s>,
}

impl<'s> Tape<'s> {
    fn new(current: Match<'s>, next: Match<'s>) -> Tape<'s> {
        Tape {
            current: current,
            next: next,
        }
    }
    fn _next(&mut self, pattern: char) -> Match<'s> {
        let m = self.next;
        (*self) = m.split_ounce(pattern);
        m
    }
}

impl<'s> Match<'s> {
    fn new(string: &'s str) -> Match {
        Match::Some(string)
    }
    fn len(&self) -> usize {
        match self {
            Match::Some(s) => s.len(),
            Match::None => 0,
        }
    }
    fn _split(self, pattern: char) -> Vec<Match<'s>> {
        let mut v = Vec::new();
        let mut tape = self.split_ounce(pattern);
        loop {
            v.push(tape.current);
            if let Match::Some(_) = tape.next {
                tape = tape.next.split_ounce(pattern);
            } else {
                break v;
            }
        }
    }
    fn _split_for(self, f: &mut impl FnMut(char) -> bool) -> Tape<'s> {
        if let Match::Some(s) = self {
            for (i, c) in s.to_string().chars().enumerate() {
                if f(c) {
                    let (left, right) = s.split_at(i);
                    return Tape::new(Match::new(left), Match::new(right));
                }
            }
        }
        Tape::new(self, Match::None)
    }
    fn split_ounce(self, pattern: char) -> Tape<'s> {
        let (mut paren, mut brace) = (0, 0);
        if let Match::Some(s) = self {
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
                        Match::Some(&s[0..i].trim()),
                        if &s[i + 1..] != "" {
                            Match::new(&s[i + 1..].trim())
                        } else {
                            Match::None
                        },
                    );
                }
            }
        }
        Tape::new(self, Match::None)
    }
    fn clamp(&self, opening: &'s str, closing: &'s str) -> Match<'s> {
        let (mut start, mut brace) = (0, 0);
        if let Match::Some(s) = self {
            for i in 0..s.len() {
                if &s[i..i + opening.len()] == opening {
                    if brace == 0 {
                        start = i;
                    }
                    brace += 1;
                } else if &s[i..i + closing.len()] == closing {
                    brace -= 1;
                    if brace == 0 {
                        return Match::new(&s[start + 1..i].trim());
                    }
                }
            }
        }
        Match::None
    }
    fn filter(&self, pattern: char, mut f: impl FnMut(Match) -> Result<(), &'static str>) {
        let tape = self.split_ounce(pattern);
        match f(tape.current) {
            Ok(_) => {
                if let Match::Some(_) = tape.next {
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
        if let Match::Some(s) = self {
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
                        let tape = Match::new(name).clamp("{", "}").split_ounce(':');
                        Layout::Recursive {
                            outer: { Box::new(layout(tape.current.release())) },
                            inner: {
                                let mut vec = Vec::new();
                                tape.next.filter(' ', |s| {
                                    if s.len() == s.clamp("{", "}").len() + 2
                                        || s.len() == s.clamp("(", ")").len() + 2
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
                        let tape = Match::new(name).clamp("(", ")").split_ounce(' ');
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
