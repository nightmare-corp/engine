pub use ne_log as L;
use L::tracing;

#[allow(dead_code)]
pub use tracing::{debug, error, info, trace, warn};

/// print to console that will be disabled on release!
/// example: log("hello", "int:", number, "string:", str);
/// TODO add ne_log
/// TODO I want this to be usuable by println!() and replace all cases of println!() with ne::log!
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
        println!("{}", result);
    };
}
#[cfg(not(any(debug_assertions, ne_log)))]
#[macro_export]
macro_rules! log {
    ($($args:expr),*) => {
    };
}