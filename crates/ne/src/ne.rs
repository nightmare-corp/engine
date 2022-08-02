use L::tracing;
pub use ne_log as L;
// pub use ne_files as F;
// pub use engine_bench as B;

#[allow(dead_code)]
pub use tracing::{info,debug,trace,warn,error};

/// initialze logging functionality
// this is messy
#[macro_export]
macro_rules!
init_log {
    ($level:expr) => {
    //TODO move this too ne_log. Maybe as a macro otherwise as a function with params. Name it L::init_logger(...)
    //use params maybe: filter: tracing::Level, rolling_file_appender, non_blocking, _guard...
    if cfg!(debug_assertions) {
        let rolling_file_appender = ne::L::tracing_appender::rolling::daily(
            "logs",
            "log.log",
        );
        let (non_blocking, _guard) = ne::L::tracing_appender::non_blocking(rolling_file_appender);
    
        tracing_subscriber::fmt()
        .with_max_level($level)
        .with_writer(non_blocking)
        .init();
    
        trace!("Initialized logging [TRACE]");
        debug!("Initialized logging [DEBUG]");
        info!("Initialized logging [INFO]");
        warn!("Initialized logging [WARN]");
    }
    else {
            let rolling_file_appender = ne::L::tracing_appender::rolling::daily(
                "logs",
                "log.log",
            );
            let (non_blocking, _guard) = ne::L::tracing_appender::non_blocking(rolling_file_appender);
            tracing_subscriber::fmt()
            .with_max_level( tracing::Level::ERROR)
            .with_writer(non_blocking)
            .init();
        }
    };
}