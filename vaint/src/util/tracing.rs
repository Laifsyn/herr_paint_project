/// Function to initialize a specific tracing subscriber for the application.
pub fn init() {
    use tracing_subscriber::filter::{EnvFilter, LevelFilter};

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::builder().with_default_directive(LevelFilter::INFO.into()).from_env_lossy())
        .init();
}
