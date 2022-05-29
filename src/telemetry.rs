use tracing::{Subscriber, subscriber::set_global_default};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{EnvFilter, Registry, layer::SubscriberExt, fmt::MakeWriter};


/// Compose multple layers into a `tracing` subscriber
/// 
/// # Implemenetation notes
/// 
/// We are using `impl Subscriber` as return type to avoid having
/// to speel out the concrete type of the subscriber, which can be
/// complex.
/// `Send` and `Sync` are modatory to make it possible to pass it to
/// `init_subscriber`
pub fn get_subscriber<Sink>(
  name: String,
  env_filter: String,
  sink: Sink,
) -> impl Subscriber + Send + Sync 
  where 
  Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static {
  // printing all spans at info-level or above
  // if the RUST_LOG env varaible has not been set
  let env_filter = EnvFilter::try_from_default_env()
    .unwrap_or_else(|_| EnvFilter::new(env_filter));
  let formatting_layer = BunyanFormattingLayer::new(
    name,
    sink
  );

  Registry::default()
    .with(env_filter)
    .with(JsonStorageLayer)
    .with(formatting_layer)
}

pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
  // Redirect all log events to tracing Subscriber
  LogTracer::init().expect("Failed to set logger");

  set_global_default(subscriber).expect("Failed to set subscriber.");
}