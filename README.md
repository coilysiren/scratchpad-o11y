# o11y-scratchpad

## Goals

o11y staack with:

- Grafana (single UI + plugins for Tempo & Pyroscope)
- Prometheus
- Tempo
- Pyroscope
- OpenTelemetry Collector
- All UIs exposed independently (Grafana + native UIs)

## Folder Structure

├── docker-compose.yml
├── prometheus.yml
├── tempo.yaml
├── otel-collector-config.yaml
├── py-server/
│   ├── server.py
│   ├── requirements.txt
│   └── Dockerfile
└── rust-server/
    ├── Dockerfile
    └── main.rs
