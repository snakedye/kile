use super::client::*;
use super::layout::*;

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
            let mut value = value.split_whitespace();
            loop {
                match value.next() {
                    Some(fields) => {
                        let mut fields = fields.split('|');
                        let tags = match fields.next() {
                            Some(tag) => match tag {
                                "focused" => output_handle.focused..output_handle.focused + 1,
                                "all" => 0..32,
                                _ => match tag.parse::<usize>() {
                                    Ok(int) => {
                                        if int > 0 && int < 33 {
                                            int - 1..int
                                        } else {
                                            break;
                                        }
                                    }
                                    Err(_) => break,
                                },
                            },
                            None => {
                                break;
                            }
                        };
                        let layout = layout(fields.next().unwrap_or_default());
                        let main_amount = fields.next().unwrap_or_default().parse::<u32>();
                        let main_factor = fields.next().unwrap_or_default().parse::<f64>();
                        let main_index = fields.next().unwrap_or_default().parse::<u32>();
                        let window_rule = match fields.next() {
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
                    None => break,
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

fn wrap<'s>(string: &'s str, opening: &str, closing: &str) -> (&'s str, usize) {
    let mut start = 0;
    let mut end = 0;
    let mut pattern = 0;
    let mut captured = "";
    for i in 0..string.len() {
        if &string[i..i+opening.len()] == opening {
            pattern += 1;
            if pattern == 0 {
                start = i + 1;
            }
        } else if &string[i..i+closing.len()] == closing {
            pattern -= 1;
            if pattern == 0 {
                end = i;
                captured = &string[start + 1..end];
                break;
            }
        }
    }
    (captured, end)
}

fn layout(name: &str) -> Layout {
    match name {
        "v" | "ver" | "vertical" => Layout::Vertical,
        "h" | "hor" | "horizontal" => Layout::Horizontal,
        "t" | "tab" | "tabbed" => Layout::Tab,
        "d" | "dwd" | "dwindle" => Layout::Dwindle(0),
        "D" | "Dwd" | "Dwindle" => Layout::Dwindle(1),
        "f" | "ful" | "full" => Layout::Full,
        _ => {
            let captured = wrap(name, "{", "}");
            let closure = captured.0;
            match closure {
                "" => Layout::Full,
                _ => {
                    let mut outer = "";
                    let mut inner = "";
                    for i in 0..closure.len() {
                        let c = &closure[i..i + 1];
                        match c {
                            "{" => {
                                let nested = wrap(name, "{", "}");
                                outer = &closure[i..nested.1 + 1];
                                inner = &closure[i + nested.1 + 2..];
                                break;
                            }
                            ":" => {
                                outer = &closure[0..i];
                                inner = &closure[i + 1..];
                                break;
                            }
                            _ => {}
                        }
                    }
                    Layout::Recursive {
                        outer: { Box::new(layout(outer)) },
                        inner: {
                            let mut vec = Vec::new();
                            let mut i = 0;
                            while i < inner.len() {
                                let char = &inner[i..i + 1];
                                match char {
                                    "{" => {
                                        let nested = wrap(name, "{", "}");
                                        if nested.1 == 0 {
                                            break;
                                        } else if nested.1 < 2 {
                                            i += 1;
                                            continue;
                                        } else {
                                            vec.push(layout(&inner[i..]));
                                            i += nested.1;
                                        }
                                    }
                                    " " | "}" => i += 1,
                                    _ => {
                                        vec.push(layout(char));
                                        i += 1
                                    }
                                }
                            }
                            vec
                        },
                    }
                }
            }
        }
    }
}
