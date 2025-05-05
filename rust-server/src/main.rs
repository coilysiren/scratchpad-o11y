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
    info!("Starting application");

    // Initialize structured logger with INFO level
    tracing_subscriber::fmt()
        .json()
        .with_max_level(tracing::Level::INFO)
        .init();

    let profiler = PyroscopeAgent::builder("http://pyroscope:4040", "rust-server")
        .backend(pprof_backend(PprofConfig::new().sample_rate(100)))
        .build()
        .unwrap();

    let profiler_running = profiler.start().map_err(
        |e| std::io::Error::new(std::io::ErrorKind::Other, e)
    )?;

    let server = HttpServer::new(|| App::new()
        .wrap(middleware::RequestLogger)
        .service(index)
        .service(hello))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await;

    let profiler_stopped = profiler_running.stop().map_err(
        |e| std::io::Error::new(std::io::ErrorKind::Other, e)
    )?;
    profiler_stopped.shutdown();

    server
}
