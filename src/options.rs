use super::display::{Context, Frame, State, Tag};
use crate::wayland::{
    river_layout_unstable_v1::{zriver_layout_v1, zriver_layout_v1::ZriverLayoutV1},
    river_options_unstable_v1::zriver_option_handle_v1,
    river_status_unstable_v1::{
        zriver_output_status_v1,
        zriver_seat_status_v1,
    },
};
use wayland_client::DispatchData;
use wayland_client::Main;

#[derive(Clone, Debug)]
pub struct Options {
    pub state: bool,
    pub serial: u32,
    pub tagmask: u32,
    pub zlayout: Option<Main<ZriverLayoutV1>>,
    pub view_amount: u32,
    pub window_title: String,
    pub usable_width: u32,
    pub usable_height: u32,
    pub view_padding: u32,
    pub outer_padding: u32,
    pub main_index: u32,
    pub main_factor: f64,
    pub main_amount: u32,
    pub layout_window: String,
    pub layout_frame: String,
}

#[derive(Copy, Clone)]
pub union Value {
    double: f64,
    uint: u32,
    int: i32,
}

#[derive(Copy, Clone, Debug)]
pub enum Layout {
    Tab,
    Full,
    Vertical,
    Horizontal,
}

impl Options {
    pub fn new() -> Options {
        return {
            Options {
                state: true,
                serial: 0,
                tagmask: 0,
                zlayout: None,
                window_title: String::new(),
                view_amount: 0,
                view_padding: 0,
                outer_padding: 0,
                usable_width: 0,
                usable_height: 0,
                main_factor: 0.0,
                main_index: 0,
                main_amount: 0,
                layout_window: String::new(),
                layout_frame: String::new(),
            }
        };
    }
    // Listen to the options and layout and returns an Options when the context is updated
    pub fn init(&mut self, context: Context) {
        let new_context = context.clone();

        self.zlayout = Some(
            new_context
                .layout_manager
                // .unwrap()
                .expect("Compositor doesn't implement ZriverOptionsManagerV1")
                .get_river_layout(&new_context.output.unwrap(), new_context.namespace),
        );
        self.clone().zlayout.unwrap().quick_assign(
            move |_, event, mut option: DispatchData| match event {
                zriver_layout_v1::Event::LayoutDemand {
                    view_amount,
                    usable_width,
                    usable_height,
                    serial
                } => {
                    option.get::<Options>().unwrap().serial = serial;
                    option.get::<Options>().unwrap().view_amount = view_amount;
                    option.get::<Options>().unwrap().usable_height = usable_height;
                    option.get::<Options>().unwrap().usable_width = usable_width;
                }
                zriver_layout_v1::Event::AdvertiseView {
                    tags,
                    app_id,
                    serial,
                } => {}
                zriver_layout_v1::Event::NamespaceInUse => {
                    println!("Namespace already in use.");
                    option.get::<Options>().unwrap().state = false;
                }
                zriver_layout_v1::Event::AdvertiseDone { serial } => {}
            },
        );

        let seat_status = context
            .status_manager
            .clone()
            .expect("Compositor doesn't implement river_status_unstable_v1")
            .get_river_seat_status(&new_context.seat.clone().unwrap());

        seat_status.quick_assign(move |_, event, mut options| {
            match event {
                zriver_seat_status_v1::Event::FocusedView { title }=> {
                    options.get::<Options>().unwrap().window_title = title;
                }
                zriver_seat_status_v1::Event::FocusedOutput { output }=> { }
                zriver_seat_status_v1::Event::UnfocusedOutput { output }=> { }
            }
        });

        let output_status = context
            .status_manager
            .clone()
            .expect("Compositor doesn't implement river_status_unstable_v1")
            .get_river_output_status(&context.output.clone().unwrap());

        output_status.quick_assign(move |_, event, mut options| {
            match event {
                zriver_output_status_v1::Event::FocusedTags { tags }=>{
                    options.get::<Options>().unwrap().tagmask = tags;
                    match &options.get::<Options>().unwrap().zlayout {
                        Some(zlayout) => zlayout.parameters_changed(),
                        // Some(zlayout) => {},
                        None => {}
                    }
                }
                zriver_output_status_v1::Event::ViewTags { tags }=>{}
            }
        });

        self.get_option("main_factor", &context);
        self.get_option("main_amount", &context);
        self.get_option("main_index", &context);
        self.get_option("view_padding", &context);
        self.get_option("outer_padding", &context);
        self.get_option("kile_frame", &context);
        self.get_option("kile_window", &context);
    }
    fn get_option(&mut self, name: &'static str, context: &Context) {
        let option = context
            .options_manager
            .clone()
            .expect("Compositor doesn't implement river_options_unstable_v1")
            .get_option_handle(name.to_owned(), Some(&context.output.as_ref().unwrap()));
        option.quick_assign(move |_, event, mut options| {
            let mut option_value: Value = Value { uint: 0 };
            let mut args: String = String::new();
            match event {
                zriver_option_handle_v1::Event::StringValue { value } => {
                    args = value.unwrap();
                }
                zriver_option_handle_v1::Event::FixedValue { value } => option_value.double = value,
                zriver_option_handle_v1::Event::UintValue { value } => option_value.uint = value,
                zriver_option_handle_v1::Event::IntValue { value } => {
                    if value < 0 {
                        option_value.int = 0;
                    } else {
                        option_value.int = value;
                    }
                }
                zriver_option_handle_v1::Event::Unset => {}
            }
            match &options.get::<Options>().unwrap().zlayout {
                Some(zlayout) => zlayout.parameters_changed(),
                None => {}
            }
            unsafe {
                match name {
                    "main_index" => options.get::<Options>().unwrap().main_index = option_value.uint,
                    "main_amount" => options.get::<Options>().unwrap().main_amount = option_value.uint,
                    "main_factor" => {
                        options.get::<Options>().unwrap().main_factor = option_value.double
                    }
                    "view_padding" => {
                        options.get::<Options>().unwrap().view_padding = option_value.uint
                    }
                    "outer_padding" => {
                        options.get::<Options>().unwrap().outer_padding = option_value.uint
                    }
                    "kile_frame" => {
                        options.get::<Options>().unwrap().layout_frame = args;
                    }
                    "kile_window" => {
                        options.get::<Options>().unwrap().layout_window = args;
                    }
                    _ => {}
                }
            }
        });
    }
    pub fn layout_frame(&self, layout_frame: String, count: u32)->(Layout, u32, State) {
        let mut orientation = Layout::Full;
        let total_views = if self.view_amount >= count as u32 {
            if self.view_amount > self.main_amount 
            && self.view_amount - self.main_amount + 1 < count - 1 {
                count as u32 - (self.view_amount - self.main_amount)
            } else {
                count as u32
            }
        } else {
            self.view_amount
        };
        for (i, c) in layout_frame.chars().enumerate() {
            match c {
                'v' => orientation = Layout::Vertical,
                'h' => orientation = Layout::Horizontal,
                't' => orientation = Layout::Tab,
                'f' => orientation = Layout::Full,
                _ => println!("{}: Not a valid character", i),
            }
        }
        (orientation, total_views, State::Frame)
    }
    pub fn layout_window(&self, string: String, total_views: u32) -> Vec<(Layout, u32, State)> {
        let mut layout = Vec::new();
        let mut orientation = Layout::Full;
        let main_view =
            if self.main_index + self.main_amount < self.view_amount && self.main_amount > 0 {
                1
            } else {
                0
            };
        let main_amount = if self.main_index + self.main_amount > self.view_amount {
            self.view_amount - self.main_index
        } else {
            self.main_amount
        };
        let mut reste = 0;

        for (i, c) in string.chars().enumerate() {
            match c {
                'v' => orientation = Layout::Vertical,
                'h' => orientation = Layout::Horizontal,
                't' => orientation = Layout::Tab,
                'f' => orientation = Layout::Full,
                _ => println!("{}: Not a valid character at index {}", c, i),
            }
            if i == self.main_index as usize && self.main_amount > 0 {
                layout.push((orientation, main_amount, State::Window));
            } else {
                let mut client_count = if total_views > 0 {
                    (self.view_amount - main_amount) / (total_views - main_view)
                } else {
                    self.view_amount
                };
                if reste > 0 {
                    client_count += 1;
                    reste -= 1;
                }
                layout.push((orientation, client_count, State::Window));
            }
            if i > total_views as usize {
                break;
            }
        }

        return layout;
    }
    pub fn push_dimensions(&self, frame: &Frame) {
        self.zlayout.clone().unwrap().push_view_dimensions(
            self.serial,
            frame.x as i32,
            frame.y as i32,
            frame.w,
            frame.h,
        )
    }
    pub fn debug(&self) {
        println!("Option - {}", self.serial);
        println!("\n  ZriverLayoutV1");
        println!("    view_amount : {}", self.view_amount);
        println!("    usable_width : {}", self.usable_width);
        println!("    usable_height : {}", self.usable_height);
        println!("\n  ZriverOptionHandleV1");
        println!("    outer_padding : {}", self.outer_padding);
        println!("    view_padding : {}", self.view_padding);
        println!("    main_factor : {}", self.main_factor);
        println!("    main_index : {}", self.main_index);
        println!("    main_amount : {}", self.main_amount);
        println!("    layout : {}", self.layout_window);
        println!("\n  ZriverOutputStatusV1");
        println!("    tagmask : {}", self.tagmask);
        println!("\n  ZriverSeatStatusV1");
        println!("    window_title : {}\n", self.window_title);
    }
    pub fn commit(&self) {
        self.zlayout.clone().unwrap().commit(self.serial);
    }
}
