// use std::env;
//TODO cleanup dependencies remove tracing_subscriber
use chrono::Utc;
use nightmare_engine::*;

// mod projectmacros;
use ne_app1::{App, Plugin};

//----------------------------------------------------------------------------------

fn main() {
    // env::set_var("RUST_BACKTRACE", "1");
    L::init_log!(tracing::Level::INFO);

    //We are always gonna use as many threads as we can?
    //Nah that sounds dangerous... minimum of 3 threads.
    //after that it's using total threads -2.
    
//how do I simply run a function over and over again on the main thread? ``non-send``
    App::new()
    //TODO
    // .add_plugin(Logger)
    //TODO
    .add_plugin(Renderer)


    //TODO this isn't called???
    .add_running(test_running)

    // .insert_non_send_resource(l1)

    //TODO
    // .init_resource
    // .init_non_send_resource
    // .

    .run(); 
}

//prints every 2 seconds.
fn test_running()
{
    let t = Utc::now().time();
    println!("{:?}", t);
    
    // std::thread::sleep(time::Duration::from_secs(2));
}


fn main_loop(app:App)
{
    nightmare_engine::run_engine( "ne_editor");
}
struct Renderer;
impl Plugin for Renderer
{
    fn setup(&self, app: &mut App) {
        app.set_runner(main_loop);
    }
}

struct Logger;
impl Plugin for Logger
{
    fn setup(&self, app: &mut App)
    {
        //this is annoying... because we neeed certain variablesss to outlive this function inside main..?
    }
}
//----------------------------------------------------------------------------------

//TODO
// pub struct HelloPlugin;
// impl Plugin for HelloPlugin {
//     fn setup(&self, app: &mut App) {
//         app
//         .add_startup_func(add_people)
//         .add_running(greet_people);
//     }
// }

// struct GreetTimer(Timer);

// fn greet_people(
//     time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
//     // update our timer with the time elapsed since the last update
//     // if that caused the timer to finish, we say hello to everyone
//     if timer.0.tick(time.delta()).just_finished() {
//         for name in query.iter() {
//             println!("hello {}!", name.0);
//         }
//     }
// }

//-------------------------------------------------------------------------------------------

// pub struct Logger;
// impl Plugin for Logger {
//     fn setup(&self, &mut App) {
        
//     }
// }

//TODO return two variables so that logging can continue.
//problem logging stops once certain object are out of scope.
// fn log_init()
// {
//     L::init_log!(tracing::Level::INFO);
    
// }