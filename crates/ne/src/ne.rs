pub use ne_log as L;
use L::tracing;

#[allow(dead_code)]
pub use tracing::{debug, error, info, trace, warn};


/// multiple arguments very similar to ne::log!: log!("{} {} {} {}", "exactly the same: ", 163, 136.0, my_var);
/// set environment variable neprint for this to work std::env::set_var("neprint", "true");
#[macro_export]
macro_rules! log {
    () => {
        $crate::print!("\n")
        //I really want this? to work:
        // cargo run -p frame_counter --release --features "ne_log"
        todo!()
    };
    //but for one arg it will simply print that arg as with {:?} the debug setting.
    ($arg:tt) => {
    if std::env::var("neprint").is_ok() {
        println!("{}", format!("{:?}", $arg));
    }

    };
    ($($arg:tt)*) => {
    if std::env::var("neprint").is_ok() {
        println!($($arg)*);
    }
    };
}
/* 
//OLD implementation
/// print to console that will be disabled on release!
/// example: log("hello", "int:", number, "string:", str);
#[cfg(any(debug_assertions, "ne_log"))]
#[macro_export]
macro_rules! log {
    ($($args:expr),*) => {
        let mut result: String = String::from("");
        $(
            //usual print
            // let tempstr: String = format!("{}", format_args!("{}", $args));
            // result.push_str(&tempstr[..]);

            //debug print for types don't implement fmt::display
            let tempstr: String = format!("{:?}", format_args!("{:?}", $args));
            result.push_str(&tempstr[..]);
        )*
        ne::log!("{}", result);
    };
}
*/