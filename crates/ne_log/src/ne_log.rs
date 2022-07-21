
//cfg to only use during debug and remove when using release mode. Will decrease size by a bit? TODO measure
pub use tracing as tracing;
pub use tracing::{info,debug,trace,warn};
pub use tracing_appender::rolling::RollingFileAppender;

// pub use tracing_subscriber as tracing_subscriber;
pub use tracing_appender as tracing_appender;


// // pub fn init_logger(filter: tracing::Level)
// #[macro_export]
// macro_rules! 
// init_logger
// {
//     //filter: tracing::Level
//     //example params: tracing::Level::ERROR, tracing::Level::WARN, tracing::Level::INFO 
//     ($arg:expr) =>
//     {
//         let rolling_file_appender = tracing_appender::rolling::daily(
//             "logs",
//             "log.log",
//         );
    
//         let (non_blocking, _guard) = tracing_appender::non_blocking(rolling_file_appender);
    
//         tracing_subscriber::fmt()
//         .with_max_level(arg)
//         .with_writer(non_blocking)
//         .init();
    
//         trace!("Initialized logging [TRACE]");
//         debug!("Initialized logging [DEBUG]");
//         info!("Initialized logging [INFO]");
//         warn!("Initialized logging [WARN]");
//     }
// }

//TODO
// pub fn init_logger(filter: tracing::Level) -> RollingFileAppender
// {
//     let rolling_file_appender = tracing_appender::rolling::daily(
//         "logs",
//         "log.log",
//     );

//     let (non_blocking, _guard) = tracing_appender::non_blocking(rolling_file_appender);

//     tracing_subscriber::fmt()
//     .with_max_level(filter)
//     .with_writer(non_blocking)
//     .init();

//     trace!("Initialized logging [TRACE]");
//     debug!("Initialized logging [DEBUG]");
//     info!("Initialized logging [INFO]");
//     warn!("Initialized logging [WARN]");


//     // I need to return this
//     rolling_file_appender

//     //and maybe this too: 
//     //(non_blocking, _guard) 

// }


//TODO cfg to use println! instead of tracing during release?
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

/* }
    /// Legacy code:
    #[macro_export]
    macro_rules! log {
        ($($args:expr),*) => {
            let mut result: String = String::from("");
            $(
                let tempstr: String = format!("{}", format_args!("{}", $args));
                result.push_str(&tempstr[..]);
            )*
            println!("{}", result);
        };
    } */

