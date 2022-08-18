// use std::env;
//TODO cleanup dependencies remove tracing_subscriber

use std::time;
use chrono::Utc;
//TODO I want this moved to nightmare_engine prelude somehow?
//This does feel confusing but it is unacceptable to rename ne_log to L
//I want it to automatically be included as L everywhere!
use ne::{L, warn, info};
// mod projectmacros;
use ne_app1::{App, Plugin};

/*#[derive(Component)]
struct Position { x: f32, y: f32 }
#[derive(Component)]
struct Velocity { x: f32, y: f32 }

// This system moves each entity with a Position and Velocity component
fn movement(mut query: Query<(&mut Position, &Velocity)>) {
    // for (mut position, velocity) in &mut query {
    //     position.x += velocity.x;
    //     position.y += velocity.y;
    // }
}
fn printsome()
{
    println!("some");
}

fn main() {
    // Create a new empty World to hold our Entities and Components
    let mut world = World::new();

    // Spawn an entity with Position and Velocity components
    world.spawn()
        .insert(Position { x: 0.0, y: 0.0 })
        .insert(Velocity { x: 1.0, y: 0.0 });

    // Create a new Schedule, which defines an execution strategy for Systems
    let mut schedule = Schedule::default();

    // Add a Stage to our schedule. Each Stage in a schedule runs all of its systems
    // before moving on to the next Stage
    schedule.add_stage("update", SystemStage::parallel()
        .with_system(movement)
        .with_system(printsome)
    );

    // Run the schedule once. If your app has a "loop", you would run this once per loop
    while(true)
    {
        schedule.run(&mut world);
    }
}*/

//----------------------------------------------------------------------------------

fn main() {
    // env::set_var("RUST_BACKTRACE", "1");
    // nightmare_engine::run_engine(tracing::Level::INFO, "ne_editor");

    //We are always gonna use as many threads as we can?
    //Nah that sounds dangerous... minimum of 3 threads.
    //after that it's using total threads -2.

    App::new()
    .add_plugin(renderer)

    //TODO call every 2 seconds
    .add_running(test_running)

    .run();
}

//prints every 2 seconds.
fn test_running()
{
    let t = Utc::now().time();
    println!("{:?}", t);
    // std::thread::sleep(time::Duration::from_secs(2));
}

struct renderer;
impl Plugin for renderer
{
    fn setup(&self, app: &mut App) {
        fn l1() {
            //how to do this? 
            //We need to initalize the renderer
            //And then render once per tick.

            nightmare_engine::run_engine(tracing::Level::INFO, "ne_editor");

            
        }
        app.add_running(l1);

        //for now add this as a seconds tick thread.

        // nightmare_engine::app1::run_engine(tracing::Level::INFO, "ne_editor");
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