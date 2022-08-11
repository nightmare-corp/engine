// use std::env;

// mod projectmacros;

use nightmare_engine::*;
fn main() {
    // env::set_var("RUST_BACKTRACE", "1");

    //TODO cleanup dependencies remove tracing_subscriber
    L::init_log!(tracing::Level::INFO);
    run_engine( 
        tracing::Level::INFO,
         "Nightmare_Editor");
    
    App::new()
    .add_func(test_run)
    .add_plugin(renderer)
    .run();
}

fn test_run()
{
    println!("test success");
}

pub struct Logger;
impl Plugin for Logger {
    fn setup(&self, &mut App) {
        
    }
}

//TODO return two variables so that logging can continue.
//problem logging stops once certain object are out of scope.
// fn log_init()
// {
//     L::init_log!(tracing::Level::INFO);
    
// }