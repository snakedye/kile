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
            Some(rule) => {
                let mut rule = rule.split(':');
                let tags = match rule.next() {
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
                let outer_layout = outer_layout(rule.next().unwrap_or_default().to_string());
                let inner_layout = inner_layout(rule.next().unwrap_or_default().to_string());
                let window_rule = match rule.next() {
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
                            if let Some(outer_layout) = outer_layout {
                                tag.outer = outer_layout;
                            }
                            if let Some(inner_layout) = inner_layout.clone() {
                                tag.inner = inner_layout;
                            }
                            tag.options.rule = window_rule.clone();
                        }
                        None => {
                            let mut options = Options::new();
                            options.rule = window_rule.clone();
                            output_handle.tags[i] = Some({
                                Tag {
                                    options: options,
                                    outer: outer_layout.unwrap_or(Layout::Full),
                                    inner: inner_layout.clone().unwrap_or(vec![Layout::Full]),
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

pub fn outer_layout(layout_output: String) -> Option<Layout> {
    match layout_output.chars().next() {
        Some(c) => Some(layout(c)),
        None => None,
    }
}
pub fn inner_layout(string: String) -> Option<Vec<Layout>> {
    let mut vec = Vec::new();

    for c in string.chars() {
        vec.push(layout(c));
    }

    if vec.len() > 0 {
        Some(vec)
    } else {
        None
    }
}
fn layout(c: char) -> Layout {
    match c {
        'v' => Layout::Vertical,
        'h' => Layout::Horizontal,
        't' => Layout::Tab,
        'd' => Layout::Recursive { modi: 0 },
        'D' => Layout::Recursive { modi: 1 },
        'f' => Layout::Full,
        _ => {
            println!("{}: Invalid character", c);
            Layout::Full
        }
    }
}
