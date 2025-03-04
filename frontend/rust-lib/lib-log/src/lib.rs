mod layer;

use log::LevelFilter;

use tracing::subscriber::set_global_default;

use crate::layer::*;
use lazy_static::lazy_static;
use std::sync::RwLock;
use tracing_appender::{non_blocking::WorkerGuard, rolling::RollingFileAppender};
use tracing_bunyan_formatter::JsonStorageLayer;
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter};

lazy_static! {
    static ref LOG_GUARD: RwLock<Option<WorkerGuard>> = RwLock::new(None);
}

pub struct Builder {
    #[allow(dead_code)]
    name: String,
    env_filter: String,
    file_appender: RollingFileAppender,
}

impl Builder {
    pub fn new(name: &str, directory: &str) -> Self {
        // let directory = directory.as_ref().to_str().unwrap().to_owned();
        let local_file_name = format!("{}.log", name);

        Builder {
            name: name.to_owned(),
            env_filter: "Info".to_owned(),
            file_appender: tracing_appender::rolling::daily(directory, local_file_name),
        }
    }

    pub fn env_filter(mut self, env_filter: &str) -> Self {
        self.env_filter = env_filter.to_owned();
        self
    }

    pub fn build(self) -> std::result::Result<(), String> {
        let env_filter = EnvFilter::new(self.env_filter);

        let (non_blocking, guard) = tracing_appender::non_blocking(self.file_appender);
        let subscriber = tracing_subscriber::fmt()
            // .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
            .with_ansi(false)
            .with_target(false)
            .with_max_level(tracing::Level::TRACE)
            .with_writer(std::io::stderr)
            .with_thread_ids(true)
            // .with_writer(non_blocking)
            .json()
            .with_current_span(true)
            .with_span_list(true)
            .compact()
            .finish()
            .with(env_filter)
            .with(JsonStorageLayer)
            .with(FlowyFormattingLayer::new(std::io::stdout))
            .with(FlowyFormattingLayer::new(non_blocking));

        // if cfg!(feature = "use_bunyan") {
        //     let formatting_layer = BunyanFormattingLayer::new(self.name.clone(),
        // std::io::stdout);     let _ =
        // set_global_default(subscriber.with(JsonStorageLayer).with(formatting_layer)).
        // map_err(|e| format!("{:?}", e))?; } else {
        //     let _ = set_global_default(subscriber).map_err(|e| format!("{:?}", e))?;
        // }

        let _ = set_global_default(subscriber).map_err(|e| format!("{:?}", e))?;
        let _ = LogTracer::builder()
            .with_max_level(LevelFilter::Trace)
            .init()
            .map_err(|e| format!("{:?}", e))?;

        *LOG_GUARD.write().unwrap() = Some(guard);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // run  cargo test --features="use_bunyan" or  cargo test
    #[test]
    fn test_log() {
        let _ = Builder::new("flowy", ".").env_filter("debug").build().unwrap();
        tracing::info!("😁  tracing::info call");
        log::debug!("😁 log::debug call");

        say("hello world");
    }

    #[tracing::instrument(name = "say")]
    fn say(s: &str) {
        tracing::info!("{}", s);
    }
}
