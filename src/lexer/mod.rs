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
            layout,
            amount,
            ratio,
            index,
        } = layout
        {
            for i in tags {
                let tag = output_handle.tags[i].as_mut();
                match tag {
                    Some(tag) => {
                        tag.layout = layout.as_ref().clone();
                        tag.name = name.to_owned();
                        tag.parameters.main_index = index;
                        tag.parameters.main_amount = amount;
                        tag.parameters.main_ratio = ratio;
                    }
                    None => {
                        output_handle.tags[i] = Some({
                            Tag {
                                name: name.to_owned(),
                                layout: layout.as_ref().clone(),
                                parameters: {
                                    Parameters {
                                        main_index: index,
                                        main_amount: amount,
                                        main_ratio: ratio,
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
                                        main_index: 0,
                                        main_amount: 1,
                                        main_ratio: 0.6,
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
