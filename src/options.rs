use super::display::{Context, Frame, State, Tag};
use crate::wayland::{
    river_layout_unstable_v1::{
        zriver_layout_manager_v1::ZriverLayoutManagerV1, zriver_layout_v1,
        zriver_layout_v1::ZriverLayoutV1,
    },
    river_options_unstable_v1::{
        zriver_option_handle_v1, zriver_option_handle_v1::ZriverOptionHandleV1,
        zriver_options_manager_v1::ZriverOptionsManagerV1,
    },
};
// use wayland_client::protocol::wl_output::WlOutput;
use wayland_client::Main;
use wayland_client::DispatchData;

#[derive(Clone, Debug)]
pub struct Options {
    pub state: bool,
    pub serial: u32,
    pub tagmask: u32,
    pub layout: Option<Main<ZriverLayoutV1>>,
    pub view_amount: u32,
    pub usable_width: u32,
    pub usable_height: u32,
    pub view_padding: u32,
    pub output_padding: u32,
    pub main_index: u32,
    pub main_factor: f64,
    pub main_count: u32,
    pub capacity: u32,
    pub arguments: String,
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
                layout: None,
                view_amount: 0,
                capacity: 0,
                view_padding: 0,
                output_padding: 0,
                usable_width: 0,
                usable_height: 0,
                main_factor: 0.0,
                main_index: 0,
                main_count: 0,
                arguments: String::from("v|h|"),
            }
        }
    }
    // Listen to the options and layout and returns an Options when the context is updated
    pub fn init(&mut self,context: Context) {

        let new_context=context.clone();

        self.layout = Some(new_context
            .layout_manager
            // .unwrap()
            .expect("Compositor doesn't implement ZriverOptionsManagerV1")
            .get_river_layout(&new_context.output.unwrap(), new_context.namespace));
        self.clone().layout.unwrap().quick_assign(move |layout_obj, event, mut option: DispatchData| match event {
            zriver_layout_v1::Event::LayoutDemand {
                view_amount,
                usable_width,
                usable_height,
                serial,
            } => {
                option.get::<Options>().unwrap().serial= serial;
                option.get::<Options>().unwrap().view_amount= view_amount;
                option.get::<Options>().unwrap().usable_height= usable_height;
                option.get::<Options>().unwrap().usable_width= usable_width;
            }
            zriver_layout_v1::Event::AdvertiseView {
                tags,
                app_id,
                serial,
            } => {}
            zriver_layout_v1::Event::NamespaceInUse => {
                println!("Namespace already in use.");
                option.get::<Options>().unwrap().state=false;
            }
            zriver_layout_v1::Event::AdvertiseDone { serial } => {}
        });

        self.get_option("main-factor", &context);
        self.get_option("main-count", &context);
        self.get_option("main-index", &context);
        self.get_option("view-padding", &context);
        self.get_option("output-padding", &context);
        self.get_option("layout", &context);

        // return self;
    }
    fn get_option(&mut self, name: &'static str, context: &Context) {
        let option = context
            .options_manager
            .clone()
            .expect("Compositor doesn't implement ZriverOptionsManagerV1")
            // .unwrap()
            .get_option_handle(name.to_owned(), Some(&context.output.as_ref().unwrap()));
        option.quick_assign(move |_, event,mut option| {
            let mut option_value: Value = Value { uint: 0 };
            let mut args: String = String::new();
            match event {
                zriver_option_handle_v1::Event::StringValue { value } => {
                    args=value.unwrap();
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
            match &option.get::<Options>().unwrap().layout {
                Some(layout)=>layout.parameters_changed(),
                None=>{}
            }
            unsafe {
                match name {
                    "main-index" => option.get::<Options>().unwrap().main_index = option_value.uint,
                    "main-count" => option.get::<Options>().unwrap().main_count = option_value.uint,
                    "main-factor" => option.get::<Options>().unwrap().main_factor = option_value.double,
                    "view-padding" => option.get::<Options>().unwrap().view_padding = option_value.uint,
                    "output-padding" => option.get::<Options>().unwrap().output_padding = option_value.uint,
                    "capacity" => option.get::<Options>().unwrap().capacity = option_value.uint,
                    "layout"=>option.get::<Options>().unwrap().arguments=args,
                    _ => {}
                }
            }
        });
    }
    pub fn parse(&self, tag:&mut Tag) -> Vec<(Layout, u32, State)> {
        let mut total_views=1;
        let mut layout=Vec::new();
        let mut orientation=Layout::Full;
        let main_view=if self.main_count > 0 {1} else {0};
        let main_count=if self.main_index +self.main_count > self.view_amount {
            self.view_amount-self.main_index
        } else { self.main_count };
        let mut reste=0;

        for (i,c) in self.arguments.chars().enumerate() {
            match c {
                'v' => orientation=Layout::Vertical,
                'h' => orientation=Layout::Horizontal,
                't' => orientation=Layout::Tab,
                'f' => orientation=Layout::Full,
                _ => println!("{}: Not a valid character at index {}", c, i),
            }
            if i == 0 {
                let client_count=if self.view_amount >= (self.arguments.len()-1) as u32 {
                    if self.view_amount-self.main_count+1 < (self.arguments.len()-1) as u32 {
                        (self.arguments.len()-1) as u32 - (self.view_amount-self.main_count)
                    } else {
                        (self.arguments.len()-1) as u32
                    }
                } else {
                    self.view_amount
                };
                total_views=client_count;
                reste=(self.view_amount-main_count)%(total_views-main_view);
                tag.reference=(orientation, client_count, State::Frame);
            } else if i-1 == self.main_index as usize {
                layout.push((orientation, main_count, State::Window));
            } else {
                let mut client_count=(self.view_amount-main_count)/(total_views-main_view);
                if reste > 0 {
                    client_count+=1;
                    reste-=1;
                }
                layout.push((orientation, client_count, State::Window));
            }
            if i > total_views as usize { break; }
        }

        return layout;
    }
    pub fn push_dimensions(&self, frame:&Frame) {
        self.layout.clone().unwrap().push_view_dimensions(
            self.serial,
            frame.x as i32,
            frame.y as i32,
            frame.w,
            frame.h,
        )
    }
    pub fn debug(&self) {
        println!("Option - {}", self.serial);
        println!("  tagmask : {}", self.tagmask);
        println!("  view_amount : {}", self.view_amount);
        println!("  usable_width : {}", self.usable_width);
        println!("  usable_height : {}", self.usable_height);
        println!("  output_padding : {}", self.output_padding);
        println!("  output_padding : {}", self.output_padding);
        println!("  main_factor : {}", self.main_factor);
        println!("  namespace : {}\n", self.arguments);
    }
    pub fn commit(&self) {
        self.layout.clone().unwrap().commit(self.serial);
    }
}

