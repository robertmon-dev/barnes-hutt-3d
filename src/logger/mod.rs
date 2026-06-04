use tracing::{Level, span::Span};
use tracing_subscriber::EnvFilter;

pub struct Logger;

impl Logger {
    pub fn init() {
        tracing_subscriber::fmt()
            .with_env_filter(
                EnvFilter::builder()
                    .with_default_directive(Level::INFO.into())
                    .from_env_lossy(),
            )
            .with_thread_ids(true)
            .init();

        tracing::info!("Initialized logging for simulation utility");
    }

    pub fn create_frame_span(frames: usize) -> Span {
        tracing::span!(Level::INFO, "barnes_hut_frame", frame = frames)
    }
}
