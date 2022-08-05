use std::env;

mod projectmacros;
use nightmare_engine::*;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    //todo

    // include_bytes!("../cargo.toml");
    // find_file!("../cargo.toml");

    find_asset!();

    nightmare_engine::run_engine( 
        tracing::Level::INFO,
         "Nightmare_Editor");


         //TODO why is it not found??
    // let t = nightmare_engine::new();

    //TODO how to remove this??1
    app::nightmare_engine::new();

}

