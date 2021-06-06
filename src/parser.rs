use super::client::*;
use super::layout::*;

#[derive(Copy, Clone, Debug)]
struct Match<'s>{
    string: &'s str
}

impl<'s> Match<'s> {
    fn new(string: &'s str) -> Match {
        { Match {
            string: string
        } }
    }
    fn set(&mut self, string: &'s str) {
        self.string = string;
    }
    fn len(&self) -> usize {
        self.string.len()
    }
    fn split_ounce(&self, pattern: char) -> (Match, Option<Match>) {
        let mut right = None;
        let mut left = self.clone();
        let (mut brace, mut bracket) = (0, 0);
        for (i, c) in self.string.to_string().chars().enumerate() {
            match c {
                '{' => bracket+=1,
                '}' => bracket-=1,
                '(' => brace+=1,
                ')' => brace-=1,
                _ => {}
            }
            if c == pattern && brace == 0 && bracket == 0 {
                left.set(&self.string[0..i].trim());
                if &self.string[i + 1..] != "" {
                    right = Some(Match::new(&self.string[i + 1..].trim()));
                }
                break;
            }
        } 
        (left, right)
    }
    fn clamp(&self, opening: &'s str, closing: &'s str) -> Option<Match> {
        let (mut start, mut brace) = (0, 0);
        for i in 0..self.string.len() {
            if &self.string[i..i+opening.len()] == opening {
                if brace == 0 {
                    start = i;
                }
                brace += 1;
            } else if &self.string[i..i+closing.len()] == closing {
                brace -= 1;
                if brace == 0 {
                    return Some(Match::new(&self.string[start + 1..i].trim()));
                }
            }
        }
        None
    }
    fn filter(&self, pattern: char, mut f: impl FnMut(Match) -> Result<(), &'static str>) {
        let (previous, next) = self.split_ounce(pattern);
        match f(previous) {
            Ok(_) => if let Some(next) = next {
                next.filter(pattern, f);
            }
            Err(m) => if m != "" { println!("{}",m) }
        }
    }
    fn release(self) -> &'s str {
        self.string
    }
}

fn layout<'s>(name: &str) -> Layout {
    match name {
        "dec" | "deck" => Layout::Deck,
        "f" | "ful" | "full" => Layout::Full,
        "t" | "tab" | "tabbed" => Layout::Tab,
        "d0" | "dwindle" => Layout::Dwindle(0),
        "d1" | "Dwindle" => Layout::Dwindle(1),
        "v" | "ver" | "vertical" => Layout::Vertical,
        "h" | "hor" | "horizontal" => Layout::Horizontal,
        _ => if let Some(char) = name.chars().next() {
            match char {
                '{' => if let Some(r) = Match::new(name).clamp("{","}") {
                    let (outer, inner) = r.split_ounce(':');
                    Layout::Recursive {
                        outer: { Box::new(layout(outer.release())) },
                        inner: {
                            let mut vec = Vec::new();
                            if let Some(inner) = inner {
                                inner.filter(',',|s| { 
                                    if let Some(s) = inner.clamp("{","}") {
                                        if s.len() + 2 == inner.len() { return Err("") }
                                    }
                                    if let Some(s) = inner.clamp("(",")") {
                                        if s.len() + 2 == inner.len() { return Err("") }
                                    }
                                    vec.push(layout(s.release()));
                                    Ok(())
                                });
                            }
                            vec
                        },
                    }
                } else { Layout::Full }
                '(' => if let Some(s) = Match::new(name).clamp("(",")") {
                    let (layout_denominator, parameters) = s.split_ounce(';');
                    if let Some(parameters) = parameters {
                        let mut i = 0;
                        let mut var: (u32, f64, u32) = (0, 0.6, 0);
                        parameters.filter(';', |s| {
                            i += 1;
                            match i {
                                1 => match s.release().parse::<u32>() {
                                    Ok(main_amount) => {
                                        var.0 = main_amount;
                                        return Ok(())
                                    }
                                    Err(_) => return Err("Invalid main amount")
                                }
                                2 => match s.release().parse::<f64>() {
                                    Ok(main_factor) => {
                                        var.1 = main_factor;
                                        return Ok(())
                                    }
                                    Err(_) => return Err("Invalid main factor")
                                }
                                3 => match s.release().parse::<u32>() {
                                    Ok(main_index) => {
                                        var.2 = main_index;
                                        return Ok(())
                                    }
                                    Err(_) => return Err("Invalid main index")
                                }
                                _ => { Ok(()) }
                            }
                        });
                        Layout::Assisted {
                            layout: Box::new(layout(layout_denominator.release())),
                            main_amount: var.0,
                            main_factor: var.1,
                            main_index: var.2,
                        }
                    } else { Layout::Full }
                } else { Layout::Full }
                _ => Layout::Full
            }
        } else { Layout::Full }
    }
}

pub fn main<'s>(output_handle: &mut Output, name: String, value: String) {
    let mut command = value.split_whitespace();
    match name.as_ref() {
        "dimension" => {
            let mut arguments = value.split_whitespace();
            output_handle.dimension = {
                output_handle.resize = true;
                Area {
                    x: arguments.next().unwrap_or("0").parse::<u32>().unwrap(),
                    y: arguments.next().unwrap_or("0").parse::<u32>().unwrap(),
                    w: arguments.next().unwrap_or("500").parse::<u32>().unwrap(),
                    h: arguments.next().unwrap_or("500").parse::<u32>().unwrap(),
                }
            }
        }
        "resize" => {
            output_handle.resize = if let Ok(ans) = value.parse::<bool>() {
                ans
            } else {
                false
            }
        }
        "smart_padding" => {
            if let Ok(ans) = command.next().unwrap().parse::<bool>() {
                output_handle.smart_padding = ans;
            }
        }
        "set_tag" | "layout" => {
            let mut value = value.split('|');
            let tags = match value.next() {
                Some(tag) => match tag {
                    "focused" => output_handle.focused..output_handle.focused + 1,
                    "all" => 0..32,
                    _ => match tag.parse::<usize>() {
                        Ok(int) => int - 1..int,
                        Err(_) => 33..34,
                    },
                },
                None => 33..34
            };
            let layout = layout(value.next().unwrap_or_default());
            let main_amount = value.next().unwrap_or_default().parse::<u32>();
            let main_factor = value.next().unwrap_or_default().parse::<f64>();
            let main_index = value.next().unwrap_or_default().parse::<u32>();
            let window_rule = match value.next() {
                Some(app_id) => {
                    if let Ok(tag) = app_id.parse::<u32>() {
                        Rule::Tag(tag)
                    } else {
                        Rule::AppId(app_id.to_string())
                    }
                }
                None => Rule::None,
            };
            for i in tags {
                if i > 32 { break }
                let tag = output_handle.tags[i].as_mut();
                match tag {
                    Some(tag) => {
                        tag.layout = layout.clone();
                        tag.rule = window_rule.clone();
                        if let Ok(index) = main_index {
                            tag.parameters.main_index = index;
                        }
                        if let Ok(amount) = main_amount {
                            tag.parameters.main_amount = amount;
                        }
                        if let Ok(factor) = main_factor {
                            tag.parameters.main_factor = factor;
                        }
                    }
                    None => {
                        output_handle.tags[i] = Some({
                            Tag {
                                rule: window_rule.clone(),
                                parameters: { Parameters {
                                    view_padding: 5,
                                    main_index: if let Ok(index) = main_index { index } else { 1 },
                                    main_amount: if let Ok(amount) = main_amount { amount } else { 1 },
                                    main_factor: if let Ok(factor) = main_factor { factor } else { 0.55 }
                                }},
                                layout: layout.clone(),
                            }
                        })
                    }
                }
            }
        }
        "rule" => {
            if let Some(tag) = output_handle.tags[output_handle.focused].as_mut() {
                match command.next() {
                    Some(arg) => match arg {
                        "-position" => if let Some(app_id) = command.next() {
                            Rule::Position{
                                app_id: app_id.to_owned(),
                                area: { Area {
                                    x: command.next().unwrap_or("0").parse::<u32>().unwrap(),
                                    y: command.next().unwrap_or("0").parse::<u32>().unwrap(),
                                    w: command.next().unwrap_or("500").parse::<u32>().unwrap(),
                                    h: command.next().unwrap_or("500").parse::<u32>().unwrap(),
                                } }
                            };
                        }
                        "-tag" => if let Ok(uint) = command.next().unwrap_or_default().parse::<u32>() {
                            tag.rule = Rule::Tag(uint);
                        }
                        "-app-id" => {
                            tag.rule = Rule::AppId(command.next().unwrap_or_default().to_string())
                        }
                        _ => {}
                    },
                    None => tag.rule = Rule::None,
                }
            }
        }
        "clear_tag" => {
            for arg in command {
                match arg {
                    "all" => output_handle.tags = Default::default(),
                    "focused" => output_handle.tags[output_handle.focused] = None,
                    _ => match arg.parse::<usize>() {
                        Ok(int) => {
                            if int > 0 && int < 33 {
                                output_handle.tags[int - 1] = None
                            }
                        }
                        Err(_) => {}
                    },
                }
            }
        }
        _ => {}
    }
}
