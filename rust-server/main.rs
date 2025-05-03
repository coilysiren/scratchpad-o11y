use std::net::SocketAddr;
use axum::{routing::get, Router};
use tracing::{info, Level};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;
use opentelemetry::sdk::export::metrics::aggregation::cumulative_temporality_selector;
use opentelemetry_otlp::WithExportConfig;

#[tokio::main]
async fn main() {
    let otlp_exporter = opentelemetry_otlp::new_exporter().http().with_endpoint("http://otel-collector:4318");

    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(otlp_exporter)
        .install_batch(opentelemetry::runtime::Tokio)
        .expect("failed to install OTEL");

    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    let subscriber = Registry::default()
        .with(telemetry)
        .with(tracing_subscriber::fmt::layer().with_target(false).with_level(true));

    tracing::subscriber::set_global_default(subscriber).expect("failed to set global subscriber");

    let app = Router::new().route("/", get(|| async {
        info!("handling request");
        "Hello from Rust!"
    }));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8001));
    println!("Serving on http://{}", addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}
