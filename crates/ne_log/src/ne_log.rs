pub use tracing;
pub use tracing_appender::rolling::RollingFileAppender;
pub use tracing_appender;
//TODO cfg to use ne::log! instead of tracing during release?
#[macro_export]
macro_rules! err {
    //simple error and exit
    ($arg:expr) =>
    {
        let mut error_msg: String = format!("{}", format_args!("{}", $arg));
        error!("{}", error_msg);
        std::process::exit(1);
    };
    //maybe dont use this one
    ($($args:expr),*) =>
    {
        let mut error_msg: String = String::from("");
        $(
            let tempstr: String = format!("{}", format_args!("{}", $args));
            error_msg.push_str(&tempstr[..]);
        )*
        error!("{}", error_msg);
        std::process::exit(1);
    }
}

//TODO change the log format into [time]: [type] [message]
//And a debug version [time]: [where] [type] [message]
/// initializes logging, 
/// ### possible arguments:
/// tracing::Level::INFO, tracing::Level::ERROR, tracing::Level::WARN
#[macro_export]
macro_rules!
init_log {
    () => {
        let rolling_file_appender = ne::L::tracing_appender::rolling::daily(
            "logs",
            "log.log",
        );
        let (non_blocking, _guard) = ne::L::tracing_appender::non_blocking(rolling_file_appender);
        //default settings: 
        // debug_assertions -> tracing::Level::DEBUG, 
        // no debug_assertions -> tracing::Level::ERROR
        if cfg!(debug_assertions)
        {
            tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .with_writer(non_blocking)
            .init();
        }
        else{
            tracing_subscriber::fmt()
            .with_max_level(tracing::Level::ERROR)
            .with_writer(non_blocking)
            .init();
        }
        pub use tracing::{info,debug,trace,warn};

        tracing::trace!("Initialized logging [TRACE]");
        debug!("Initialized logging [DEBUG]");
        info!("Initialized logging [INFO]");
        warn!("Initialized logging [WARN]");
    };
    ($level:expr) => {
        let rolling_file_appender = ne::L::tracing_appender::rolling::daily(
            "logs",
            "log.log",
        );
        let (non_blocking, _guard) = ne::L::tracing_appender::non_blocking(rolling_file_appender);

        tracing_subscriber::fmt()
        .with_max_level($level)
        .with_writer(non_blocking)
        .init();

        pub use tracing::{info,debug,trace,warn};

        tracing::trace!("Initialized logging [TRACE]");
        debug!("Initialized logging [DEBUG]");
        info!("Initialized logging [INFO]");
        warn!("Initialized logging [WARN]");
    };
}
