use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

static mut LOG_GUARD: Option<tracing_appender::non_blocking::WorkerGuard> = None;

pub fn init() {
    let log_dir = std::env::var("LOG_DIR").unwrap_or_else(|_| ".".to_string());
    let file_appender = tracing_appender::rolling::daily(&log_dir, "app.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    
    unsafe { LOG_GUARD = Some(guard); }
    
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::fmt::layer().with_writer(non_blocking))
        .init();
}
