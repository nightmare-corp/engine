pub use ne_log as L;

/// multiple arguments very similar to ne::log!: log!("{} {} {} {}", "exactly the same: ", 163, 136.0, my_var);
/// set environment variable neprint for this to work std::env::set_var("neprint", "true");
#[macro_export]
macro_rules! log {
    () => {
        $crate::print!("\n")
        //I really want this? to work:
        // cargo run -p frame_counter --release --features "ne_log"
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
