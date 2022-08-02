use ne_files::*;
//TODO
// use nightmare_engine::prelude::*;
// use std::path::{Path, PathBuf};
// use std::env;


// fn get_cwd() -> PathBuf {

//     env::current_dir().unwrap()
// }
// fn get_asset_dir() -> PathBuf
// {
//     get_cwd().join("assets")
// }
// fn file(s:&str) -> &str
// {
//     // find_file!(s);
//     // s
//     my_proc_macro!("da");

//     ""
// }
fn main() {
    // env::set_var("RUST_BACKTRACE", "1");
    //todo
    // nightmare_engine::new();
    // nightmare_engine::run_engine( 
    //     tracing::Level::INFO,
    //      "Nightmare_Editor");
    
    //TODO is this file there? create file besides it
    // Create a `Path` from an `&'static str`

    // let mut path = env::current_dir().unwrap();
    // println!("{}", path.display());

    // path = path.join("assets");
    // path = path.join("models");
    // // assert!(path.is_file());
    // assert!(path.is_dir());

    // println!("{}", path.display());
    // let path = Path::new(".");
    // let path_d = path.display();
    // // path+="models/cube.obj";
    // println!("{}", path_d);

    //TODO
    // find_file!(get_asset_dir(), "models",
    //  "cube.obj");

    print!("hello ");
    
    my_proc_macro!(" da ");

    print!("world");
    println!();


}
