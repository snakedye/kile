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
                eprintln!("{}", e);
                Err(())
            },
        },
    };
    let data = if let Some(data) = value.split_once('\n') {
        data
    } else {
        lexer::Expression::new(value).split_ounce(' ').drop()
    };
    let layout = lexer::layout(&data.1.replace("\t", " "));
    if let Ok(tags) = tags {
        if let Layout::Parameters {
            layout,
            amount,
            factor,
            index,
        } = layout
        {
            for i in tags {
                let tag = output_handle.tags[i].as_mut();
                match tag {
                    Some(tag) => {
                        tag.layout = layout.as_ref().clone();
                        tag.name = data.0.to_owned();
                    }
                    None => {
                        output_handle.tags[i] = Some({
                            Tag {
                                name: data.0.to_owned(),
                                layout: layout.as_ref().clone(),
                                parameters: {
                                    Parameters {
                                        main_index: index,
                                        main_amount: amount,
                                        main_factor: factor,
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
                        tag.name = data.0.to_owned();
                        tag.parameters.main_index = 0;
                        tag.parameters.main_amount = 1;
                        tag.parameters.main_factor = 0.6;
                    }
                    None => {
                        output_handle.tags[i] = Some({
                            Tag {
                                name: data.0.to_owned(),
                                layout: layout.clone(),
                                parameters: {
                                    Parameters {
                                        main_index: 0,
                                        main_amount: 1,
                                        main_factor: 0.6,
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
