use super::client::*;
use super::layout::*;

pub fn main(output_handle: &mut Output, name: String, value: String) {
    let mut command = value.split_whitespace();
    match name.as_ref() {
        "smart_padding" => {
            if let Ok(ans) = command.next().unwrap().parse::<bool>() {
                output_handle.smart_padding = ans;
            }
        }
        "set_tag" => parse_tag(output_handle, value),
        "window_rule" => {
            if let Some(tag) = output_handle.tags[output_handle.focused].as_mut() {
                match command.next() {
                    Some(arg) => match arg {
                        "_tag" => {
                            if let Ok(uint) = command.next().unwrap_or_default().parse::<u32>() {
                                tag.options.rule = Rule::Tag(uint);
                            } else {
                            }
                        }
                        "_app_id" => {
                            tag.options.rule =
                                Rule::AppId(command.next().unwrap_or_default().to_string())
                        }
                        _ => {}
                    },
                    None => tag.options.rule = Rule::None,
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

fn parse_tag(output_handle: &mut Output, value: String) {
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
                println!("{:?}", layout);
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
                            tag.options.rule = window_rule.clone();
                        }
                        None => {
                            let mut options = Options::new();
                            options.rule = window_rule.clone();
                            output_handle.tags[i] = Some({
                                Tag {
                                    options: options,
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

fn brace(str: &str) -> (&str, usize) {
    let mut start = 0;
    let mut end = 0;
    let mut brace = 0;
    let mut captured = "";
    for i in 0..str.len() {
        match &str[i..i+1] {
            "{" => {
                brace+=1;
                if brace == 0 {
                    start = i+1;
                }
            }
            "}" => {
                brace-=1;
                if brace == 0 {
                    end = i;
                    captured = &str[start+1..end];
                    break;
                }
            }
            _ => {}
        }
    }
    (captured, end)
}

fn layout(name: &str) -> Layout {
    match name {
        "v" | "ver" | "vertical" => Layout::Vertical,
        "h" | "hor" | "horizontal" => Layout::Horizontal,
        "t" | "tab" | "tabbed" => Layout::Tab,
        "d" | "dwd" => Layout::Dwindle ( 0 ),
        "D" | "Dwd" => Layout::Dwindle ( 1 ),
        "f" | "ful" | "full" => Layout::Full,
        _ => {
            let captured = brace(name);
            let closure = captured.0;
            println!("closure: {}", closure);
            match closure {
                "" => Layout::Full,
                _ => {
                    let mut outer = "";
                    let mut inner = "";
                    for i in 0..closure.len() {
                        let c = &closure[i..i+1];
                        match c {
                            "{" => {
                                let nested = brace(closure);
                                outer = &closure[i..nested.1+1];
                                inner = &closure[i+nested.1+2..];
                                break;
                            }
                            ":" => {
                                outer = &closure[0..i];
                                inner = &closure[i+1..];
                                break;
                            }
                            _ => {}
                        }
                    }
                    Layout::Recursive {
                        outer: {
                            Box::new(Some(layout(outer)))
                        },
                        inner: {
                            let mut vec = Vec::new();
                            let mut i = 0;
                            while i < inner.len() {
                                let char = &inner[i..i+1];
                                match char {
                                    "{" => {
                                        let nested = brace(&inner[i..]);
                                        if nested.1 == 0 {
                                            break
                                        } else if nested.1 < 2 {
                                            i+=1;
                                            continue;
                                        } else {
                                            vec.push(layout(&inner[i..]));
                                            i += nested.1;
                                        }
                                    }
                                    " " | "}" => i+=1,
                                    _ => {
                                        println!("char: {}", char);
                                        vec.push(layout(char));
                                        i+=1
                                    }
                                }
                            }
                            vec
                        }
                    }
                }
            }
        }
    }
}
