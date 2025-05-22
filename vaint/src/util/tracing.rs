#![allow(dead_code)]
//! Utilidades para el logging de la aplicación.

/// Inicializa el logging de la aplicación.
pub fn init() {
    use tracing_subscriber::filter::{EnvFilter, LevelFilter};

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::builder().with_default_directive(LevelFilter::INFO.into()).from_env_lossy())
        .init();
}
