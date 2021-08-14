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
            Err(_) => Err(()),
        },
    };
    let mut main_index = 0;
    let mut main_amount = 1;
    let mut main_factor = 0.6;
    let layout_data = if let Some(data) = value.split_once('\n') {
        data
    } else {
        lexer::Expression::new(value).split_ounce(' ').drop()
    };
    let mut main_layout = lexer::layout(layout_data.1);
    if let Layout::Assisted {
        layout,
        amount,
        factor,
        index,
    } = main_layout
    {
        main_layout = *layout;
        main_amount = amount;
        main_factor = factor;
        main_index = index;
    }
    if let Ok(tags) = tags {
        for i in tags {
            let tag = output_handle.tags[i].as_mut();
            match tag {
                Some(tag) => {
                    tag.layout = main_layout.clone();
                    tag.name = layout_data.0.to_owned();
                    tag.parameters.main_index = main_index;
                    tag.parameters.main_amount = main_amount;
                    tag.parameters.main_factor = main_factor;
                }
                None => {
                    output_handle.tags[i] = Some({
                        Tag {
                            name: layout_data.0.to_owned(),
                            layout: main_layout.clone(),
                            parameters: {
                                Parameters {
                                    main_index,
                                    main_amount,
                                    main_factor,
                                }
                            },
                        }
                    })
                }
            }
        }
    }
}
