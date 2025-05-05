mod middleware;

use actix_web::{get, web, App, HttpServer, Responder,};
use pyroscope::{PyroscopeAgent, PyroscopeAgentBuilder};
use pyroscope_pprofrs::{pprof_backend, PprofConfig};
use std::collections::HashMap;
use std::time::Duration;
use tracing::{info, error, debug, warn};
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

    info!("Initializing Pyroscope profiler");
    
    // Define tags for better categorization
    let mut tags = HashMap::new();
    tags.insert("environment".to_string(), "development".to_string());
    tags.insert("service".to_string(), "rust-server".to_string());
    
    // Build a more comprehensive Pyroscope configuration
    let profiler_result = PyroscopeAgent::builder("http://pyroscope:4040", "rust-server")
        .backend(pprof_backend(
            PprofConfig::new()
                .sample_rate(100)
                .collection_delay(Duration::from_millis(0))
        ))
        .tags(tags)
        .upload_timeout(Duration::from_secs(10))
        .report_errors(true)
        .build();
    
    let profiler = match profiler_result {
        Ok(agent) => {
            info!("Pyroscope agent built successfully");
            agent
        },
        Err(e) => {
            error!("Failed to build Pyroscope agent: {}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("Pyroscope initialization error: {}", e)));
        }
    };

    info!("Starting Pyroscope profiler");
    let profiler_running = match profiler.start() {
        Ok(running) => {
            info!("Pyroscope profiler started successfully");
            running
        },
        Err(e) => {
            error!("Failed to start Pyroscope profiler: {}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("Pyroscope start error: {}", e)));
        }
    };

    let server = HttpServer::new(|| App::new()
        .wrap(middleware::RequestLogger)
        .service(index)
        .service(hello))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await;

    info!("Stopping Pyroscope profiler");
    let profiler_stopped = match profiler_running.stop() {
        Ok(stopped) => {
            info!("Pyroscope profiler stopped successfully");
            stopped
        },
        Err(e) => {
            error!("Failed to stop Pyroscope profiler: {}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("Pyroscope stop error: {}", e)));
        }
    };
    
    info!("Shutting down Pyroscope profiler");
    profiler_stopped.shutdown();
    info!("Pyroscope profiler shutdown complete");

    server
}
