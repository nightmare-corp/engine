// I don't know how to write tests, this will do for now!

// use core::time;

// use ne_app::{App, Schedule, World};
// use ne_render::{RenderPlugin, WindowSettings};
struct TwoDifferentSizes {
    _i_128:i128,
    _i_8:i8,
}
struct Triple128 {
    _uno:i128,
    _dos:i128,
    _tres:i128,
}
fn main() {
    ne_bench::size_of::print_size_of::<wgpu::Buffer>(); 
    ne_bench::size_of::print_size_of::<Triple128>(); 
    ne_bench::size_of::print_size_of::<TwoDifferentSizes>(); 

    ne_bench::size_of::print_size_of::<i8>(); //type: i8, size: 1
    ne_bench::size_of::print_size_of::<i32>(); //type: i32, size: 4
    ne_bench::size_of::print_size_of::<i64>(); //type: i64, size: 8
    ne_bench::size_of::print_size_of::<i128>(); //type: i128, size: 16
    ne_bench::size_of::print_size_of::<&str>(); //type: &str, size: 16
    ne_bench::size_of::print_size_of::<String>(); //type: alloc::string::String, size: 24
    ne_bench::size_of::print_size_of::<Vec<String>>(); //type: alloc::vec::Vec<alloc::string::String>, size: 24

    // ne_bench::size_of::print_size_of::<WindowSettings>(); //type: ne_render::WindowSettings, size: 128
    // ne_bench::size_of::print_size_of::<RenderPlugin>(); //type: ne_render::RenderPlugin, size: 0
    // ne_bench::size_of::print_size_of::<App>(); //type: ne_app::App, size: 808
    // ne_bench::size_of::print_size_of::<World>(); //type: bevy_ecs::world::World, size: 632
    // ne_bench::size_of::print_size_of::<Schedule>(); //type: bevy_ecs::schedule::Schedule, size: 112
}
// type: i8, size: 1
// type: i32, size: 4
// type: i64, size: 8
// type: i128, size: 16
// type: &str, size: 16
// type: alloc::string::String, size: 24
// type: alloc::vec::Vec<alloc::string::String>, size: 24