mod middleware;

use actix_web::{get, web, App, HttpServer, Responder,};
use pyroscope::PyroscopeAgent;
use pyroscope_pprofrs::{pprof_backend, PprofConfig};
use tracing::info;
use tracing_subscriber;

// pyroscope profiling
// https://github.com/grafana/pyroscope-rs

// tracing
// https://github.com/tokio-rs/tracing

#[get("/")]
async fn index() -> impl Responder {
    "Hello, World!"
}

#[get("/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize structured logger with INFO level
    tracing_subscriber::fmt()
        .json()
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("Starting application");

    let profiler = PyroscopeAgent::builder("http://localhost:4040", "myapp-profile")
        .backend(pprof_backend(PprofConfig::new().sample_rate(100)))
        .build()
        .unwrap();
    profiler.start().expect("Failed to start Pyroscope profiler");

    info!("Pyroscope profiler started");

    HttpServer::new(|| App::new()
        .wrap(middleware::RequestLogger)
        .service(index)
        .service(hello))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
