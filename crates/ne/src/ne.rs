pub use ne_log as L;
use L::tracing;

#[allow(dead_code)]
pub use tracing::{debug, error, info, trace, warn};


/// multiple arguments very similar to ne::log!: log!("{} {} {} {}", "exactly the same: ", 163, 136.0, my_var);
#[cfg(any(debug_assertions, ne_log))]
#[macro_export]
macro_rules! log {
    () => {
        $crate::print!("\n")
    };
    //but for one arg it will simply print that arg as with {:?} the debug setting.
    ($arg:tt) => {
        println!("{}", format!("{:?}", $arg));
    };
    ($($arg:tt)*) => {
        println!($($arg)*);
    };
}
#[cfg(not(any(debug_assertions, ne_log)))]
#[macro_export]
macro_rules! log {
    ($($args:expr),*) => {
    };
}

/* 
//OLD implementation
/// print to console that will be disabled on release!
/// example: log("hello", "int:", number, "string:", str);
#[cfg(any(debug_assertions, ne_log))]
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