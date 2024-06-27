use tracing::subscriber::{set_global_default, SetGlobalDefaultError};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{fmt::MakeWriter, layer::SubscriberExt, EnvFilter, Registry};

pub fn init<Sink>(name: &str, env_filter: &str, sink: Sink) -> Result<(), SetGlobalDefaultError>
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    // Redirect all `log`'s events to our subscriber
    LogTracer::init().expect("Failed to set logger redirect.");

    let env_filter = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new(env_filter));
    let formatting_layer = BunyanFormattingLayer::new(name.into(), sink);
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);

    set_global_default(subscriber)
}
