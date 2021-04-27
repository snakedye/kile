use super::client::{Output, Rule, Tag};
use super::options::{Layout, Options};

pub fn main(output_handle: &mut Output, value: String) {
    let mut command = value.split_whitespace();
    let command_name = command.next().unwrap_or_default();
    match command_name {
        "smart-padding" => {
            if let Ok(ans) = command.next().unwrap().parse::<bool>() {
                output_handle.smart_padding = ans;
            }
        }
        "set-tag" => {
            for arg in command {
                parse_tag(output_handle, arg.to_string())
            }
        }
        "window-rule" => {
            if let Some(tag) = output_handle.tags[output_handle.focused].as_mut() {
                tag.rule = match command.next() {
                    Some(app_id) => {
                        if let Ok(tag) = app_id.parse::<u32>() {
                            Rule::Tag(tag)
                        } else {
                            Rule::AppId(app_id.to_string())
                        }
                    }
                    None => Rule::None,
                };
            }
        }
        "clear-tag" => {
            for arg in command {
                match arg {
                    "all" => output_handle.tags = Default::default(),
                    "focused" => output_handle.tags[output_handle.focused] = None,
                    _ => match arg.parse::<usize>() {
                        Ok(int) => {
                            if int > 0 {
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
                let outer_layout =
                    Options::outer_layout(rule.next().unwrap_or_default().to_string());
                let inner_layout =
                    Options::inner_layout(rule.next().unwrap_or_default().to_string());
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
                            tag.rule = window_rule.clone();
                        }
                        None => {
                            output_handle.tags[i] = Some({
                                Tag {
                                    frame: None,
                                    main_index: 0,
                                    main_amount: 1,
                                    main_factor: 0.6,
                                    rule: window_rule.clone(),
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
