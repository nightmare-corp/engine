/// code partially grabbed from bevy log
use ne_app::{Plugin, App};
/* pub */ use tracing::{Level, trace, debug, info, warn, error};
use tracing_log::LogTracer;
use tracing_subscriber::{EnvFilter, Registry, prelude::__tracing_subscriber_SubscriberExt};

/// use this: ```
/// std::env::set_var("RUST_LOG", "wgpu=error, info");
/// ```
pub struct LogPlugin;
impl Default for LogPlugin {
    fn default() -> Self {
        Self {}
    }
}
impl Plugin for LogPlugin {
    fn setup(&self, app: &mut App) {
        LogTracer::init().unwrap();
        let filter_layer = EnvFilter::builder().try_from_env()
        .or_else(|_| EnvFilter::try_new("debug,wgpu=warn,naga=warn"))
            .unwrap();
        let fmt_layer = tracing_subscriber::fmt::Layer::default();
        let subscriber = Registry::default()
        .with(filter_layer)
        .with(fmt_layer);
        tracing::subscriber::set_global_default(subscriber)
        .expect("tracing::subscriber::set_global_default failed. If tracing subscriber is already set, disable LogPlugin from DefaultPlugins");
        
        trace!("Initialized logging [TRACE]");
        debug!("Initialized logging [DEBUG]");
        info!("Initialized logging [INFO]");
        warn!("Initialized logging [WARN]");
        error!("Initialized logging [ERROR]");
    }
}

#[macro_export]
macro_rules! err {
    //simple error and exit
    ($arg:expr) =>
    {
        let mut error_msg: String = format!("{}", format_args!("{}", $arg));
        error!("{}", error_msg);
        panic!();
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
        panic!();
    }
}