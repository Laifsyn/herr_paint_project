//! Utilidades para el logging de la aplicación.

/// Inicializa el logging de la aplicación.
pub fn init() {
    use tracing_subscriber::filter::{EnvFilter, LevelFilter};
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .without_time()
        .with_target(false)
        .with_file(true)
        .with_line_number(true)
        .with_env_filter(EnvFilter::builder().with_default_directive(LevelFilter::INFO.into()).from_env_lossy())
        .init();
}
