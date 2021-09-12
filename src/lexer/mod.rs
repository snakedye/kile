pub mod lexer;

use crate::client::*;
use crate::layout::*;
pub use lexer::*;

pub fn format<'s>(string: &'s str) -> (&'s str, &'s str) {
    let exp = lexer::Expression::new(string).split_ounce(' ');
    exp.drop()
}

pub fn main<'s>(output_handle: &mut Output, name: &'s str, value: &'s str) {
    let tags: Result<std::ops::Range<usize>, ()> = match name.as_ref() {
        "focused" => Ok(output_handle.focused..output_handle.focused + 1),
        "all" => Ok(0..32),
        _ => match name.parse::<usize>() {
            Ok(int) => Ok(int - 1..int),
            Err(e) => {
                eprintln!("{} : {}", e, name);
                Err(())
            }
        },
    };
    let (name, layout) = if let Some(data) = value.split_once('\n') {
        data
    } else {
        lexer::Expression::new(value).split_ounce(' ').drop()
    };
    let layout = lexer::layout(&layout.replace("\t", " "));
    if let Ok(tags) = tags {
        if let Layout::Parameters {
            ratio,
            index,
            amount,
            layout,
        } = layout
        {
            for i in tags {
                let tag = output_handle.tags[i].as_mut();
                match tag {
                    Some(tag) => {
                        tag.name = name.to_owned();
                        tag.parameters.index = index.unwrap_or(0);
                        tag.parameters.amount = amount.unwrap_or(1);
                        tag.parameters.ratio = ratio.unwrap_or(0.6);
                        tag.layout = layout.as_ref().clone();
                    }
                    None => {
                        output_handle.tags[i] = Some({
                            Tag {
                                name: name.to_owned(),
                                layout: layout.as_ref().clone(),
                                parameters: {
                                    Parameters {
                                        index: index.unwrap_or(0),
                                        amount: amount.unwrap_or(1),
                                        ratio: ratio.unwrap_or(0.6),
                                    }
                                },
                            }
                        })
                    }
                }
            }
        } else {
            for i in tags {
                let tag = output_handle.tags[i].as_mut();
                match tag {
                    Some(tag) => {
                        tag.layout = layout.clone();
                        tag.name = name.to_owned();
                    }
                    None => {
                        output_handle.tags[i] = Some({
                            Tag {
                                name: name.to_owned(),
                                layout: layout.clone(),
                                parameters: {
                                    Parameters {
                                        index: 0,
                                        amount: 1,
                                        ratio: 0.6,
                                    }
                                },
                            }
                        })
                    }
                }
            }
        }
    }
}
