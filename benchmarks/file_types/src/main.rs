// I don't know how to write tests, this will do for now!

// use core::time;

use ne_app::{get_time_passed, App};
use ne_render::{AppExit, OnRedrawRequested, OnWindowCloseRequested, RenderPlugin, WindowSettings};

fn main() {
    ne_bench::size_of::print_size_of::<i8>();               //type: i8, size: 1
    ne_bench::size_of::print_size_of::<i32>();              //type: i32, size: 4
    ne_bench::size_of::print_size_of::<i64>();              //type: i64, size: 8
    ne_bench::size_of::print_size_of::<i128>();             //type: i128, size: 16
    ne_bench::size_of::print_size_of::<&str>();             //type: &str, size: 16
    ne_bench::size_of::print_size_of::<String>();           //type: alloc::string::String, size: 24    
    ne_bench::size_of::print_size_of::<Vec<String>>();      //type: alloc::vec::Vec<alloc::string::String>, size: 24      
    ne_bench::size_of::print_size_of::<WindowSettings>();   //type: ne_render::WindowSettings, size: 128      
    ne_bench::size_of::print_size_of::<RenderPlugin>();     //type: alloc::vec::Vec<alloc::string::String>, size: 24      

}
