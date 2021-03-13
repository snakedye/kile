use super::display::Frame;

// Stores all the information I need to build a layout
pub struct Options {
    serial:i8,
    tag:i8,
    view_amount:i32,
    usable_width:i32,
    usable_height:i32,
    view_padding:i32,
    main_index:i32,
    main_factor:f32,
    main_count:f32,
    layout:Vec<Layout>,
}

#[derive(Copy, Clone, Debug)]
pub enum Layout {
    Tab,
    Full,
    Vertical,
    Horizontal,
}


impl Options {
    // Listen to the context and sets the options
    pub fn get()->Options {

        // Use the layout protocol
        let layout;

        // Use the option protocol
        let options;

        return { Options {
            serial:0,
            tag:0,
            view_amount:0,
            view_padding:0,
            usable_width:0,
            usable_height:0,
            main_factor:0.0,
            main_index:0,
            main_count:0,
            layout:Vec::new(),
        } }
    }
    // Listen to the options and layout and returns an Options when the context is updated 
    pub fn listener()->Options {

        // Use the layout protocol
        let layout;

        // Use the option protocol
        let options;

        return { Options {
            serial:0,
            tag:0,
            view_amount:0,
            view_padding:0,
            usable_width:0,
            usable_height:0,
            main_factor:0.0,
            main_index:0,
            main_count:0,
            layout:Vec::new(),
        } }
    }
    pub fn get_options(self) {

        // Use the layout protocol
        let layout;

        // Use the option protocol
        let options;

    }
    pub fn parse_layout(self, layout:&str){
        match layout {
            "tab" => self.layout.push(Layout::Tab),
            "ver" => self.layout.push(Layout::Vertical),
            "hor" => self.layout.push(Layout::Horizontal),
            "ful" => self.layout.push(Layout::Full),
            _ => {
                println!("{} isn't a valid layout", layout);
                self.layout.push(Layout::Full);
                // std::process::exit(0);
            }
        }
    }
    pub fn get_layout(self, index:i32)->&Layout {
        self.layout[index as usize]
    }
    pub fn set_options() {
    }
}
