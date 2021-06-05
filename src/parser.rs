use super::client::*;
use super::layout::*;

fn split_ounce<'s>(string: &'s str, pattern: char) -> (&'s str, Option<&'s str>) {
    let mut brace = 0;
    let mut bracket = 0;
    let (mut left, mut right) = (string, None);
    for (i, c) in string.to_string().chars().enumerate() {
        match c {
            '{' => bracket+=1,
            '}' => bracket-=1,
            '(' => brace+=1,
            ')' => brace-=1,
            _ => {}
        }
        if c == pattern && brace == 0 && bracket == 0 {
            left = &string[0..i];
            if &string[i + 1..] != "" {
                right = Some(&string[i + 1..]);
            }
            break;
        }
    } 
    (left, right)
}
fn clamp<'s>(string: &'s str, opening: &'s str, closing: &'s str) -> &'s str {
    let mut start = 0;
    let mut brace = 0;
    let mut captured = "";
    for i in 0..string.len() {
        if &string[i..i+opening.len()] == opening {
            if brace == 0 {
                start = i;
            }
            brace += 1;
        } else if &string[i..i+closing.len()] == closing {
            brace -= 1;
            if brace == 0 {
                captured = &string[start + 1..i];
                break;
            }
        }
    }
    captured
}
fn filter<'s>(string: &'s str, pattern: char, mut f: impl FnMut(&'s str)) {
    if clamp(string, "{", "}").len() + 2 == string.len() ||
        clamp(string, "(", ")").len() + 2 == string.len() {
            f(string)
    } else {
        let (previous, next) = split_ounce(string,pattern);
        f(previous);
        if let Some(next) = next {
            filter(next, pattern, f);
        }
    }
}
fn layout<'s>(name: &str) -> Layout {
    match name {
        "f" | "ful" | "full" => Layout::Full,
        "dec" | "deck" => Layout::Deck,
        "t" | "tab" | "tabbed" => Layout::Tab,
        "v" | "ver" | "vertical" => Layout::Vertical,
        "h" | "hor" | "horizontal" => Layout::Horizontal,
        "d" | "d0" | "dwindle" => Layout::Dwindle(0),
        "D" | "d1" | "Dwindle" => Layout::Dwindle(1),
        _ => if let Some(char) = name.chars().next() {
            match char {
                '{' => {
                    let (outer, inner) = split_ounce(clamp(name, "{", "}"), ':');
                    Layout::Recursive {
                        outer: { Box::new(layout(outer)) },
                        inner: {
                            let mut vec = Vec::new();
                            if let Some(inner) = inner {
                                filter(inner, ',',|s| { vec.push(layout(s)) });
                            }
                            vec
                        },
                    }
                }
                '(' => {
                    let (layout_denominator, parameters) = split_ounce(clamp(name, "(", ")"), ';');
                    if let Some(parameters) = parameters {
                        let mut assisted = { Parameters {
                            view_padding: 0,
                            main_amount: 0,
                            main_factor: 0.6,
                            main_index: 0,
                        } };
                        let mut configured = (false, false, false);
                        filter(parameters, ';', |s| {
                            if !configured.0 {
                                if let Ok(main_amount) = s.parse::<u32>() { assisted.main_amount = main_amount }
                                configured.0 = true;
                            } else if !configured.1 {
                                if let Ok(main_factor) = s.parse::<f64>() { assisted.main_factor = main_factor }
                                configured.1 = true;
                            } else if !configured.2 {
                                if let Ok(main_index) = s.parse::<u32>() { assisted.main_index = main_index }
                                configured.2 = true;
                            } 
                        });
                        Layout::Assisted {
                            layout: Box::new(layout(layout_denominator)),
                            main_amount: assisted.main_amount,
                            main_factor: assisted.main_factor,
                            main_index: assisted.main_index,
                        }
                    } else { Layout::Full }
                }
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
