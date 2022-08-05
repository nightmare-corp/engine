use std::env;

mod projectmacros;
// use nightmare_engine::get_engine_assets_dir;


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
    env::set_var("RUST_BACKTRACE", "1");
    //todo
    // nightmare_engine::new();

    // include_bytes!("../cargo.toml");
    // find_file!("../cargo.toml");

    find_asset!();

    nightmare_engine::run_engine( 
        tracing::Level::INFO,
         "Nightmare_Editor");
}

