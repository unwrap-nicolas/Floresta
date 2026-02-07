use std::net::SocketAddr;
use std::sync::atomic::AtomicU64;
use std::sync::OnceLock;

use axum::routing::get;
use axum::Router;
use prometheus_client::encoding::text::encode;
use prometheus_client::metrics::gauge::Gauge;
use prometheus_client::metrics::counter::Counter;
use prometheus_client::metrics::histogram::Histogram;
use prometheus_client::registry::Registry;
use sysinfo::System;

pub struct AppMetrics {
    registry: Registry,
    pub memory_usage: Gauge<f64, AtomicU64>,
    pub block_height: Gauge,
    pub peer_count: Gauge<f64, AtomicU64>,
    pub avg_block_processing_time: Gauge<f64, AtomicU64>,
    pub message_times: Histogram,
    pub difficulty: Gauge<f64, AtomicU64>,
    pub banned_peer_count: Counter<f64, AtomicU64>,
    pub utreexo_peer_count: Gauge<f64, AtomicU64>,
    pub block_interval: Gauge<f64, AtomicU64>,
}

impl AppMetrics {
    pub fn new() -> Self {
        let mut registry = <Registry>::default();
        let memory_usage = Gauge::<f64, AtomicU64>::default();
        let block_height = Gauge::default();
        let peer_count = Gauge::<f64, AtomicU64>::default();
        let utreexo_peer_count = Gauge::<f64, AtomicU64>::default();
        let avg_block_processing_time = Gauge::<f64, AtomicU64>::default();
        let message_times = Histogram::new([0.1, 0.5, 1.0, 2.0, 5.0, 10.0, 30.0].into_iter());
        let difficulty = Gauge::<f64, AtomicU64>::default();
        let banned_peer_count = Counter::<f64, AtomicU64>::default();
        let block_interval = Gauge::<f64, AtomicU64>::default();

        registry.register("block_height", "Current block height", block_height.clone());
        registry.register(
            "peer_count",
            "Number of connected peers",
            peer_count.clone(),
        );

        registry.register(
            "avg_block_processing_time",
            "Average block processing time in seconds",
            avg_block_processing_time.clone(),
        );

        registry.register(
            "memory_usage_gigabytes",
            "System memory usage in GB",
            memory_usage.clone(),
        );

        registry.register(
            "message_times",
            "A time-series of how long our peers take to respond to our requests. Timed out requests are not included.",
            message_times.clone(),
        );

        registry.register(
            "difficulty",
            "Difficulty of the current block",
            difficulty.clone(),
        );

        registry.register(
            "banned_peer_count",
            "Number of banned peers",
            banned_peer_count.clone(),
        );

        registry.register(
            "utreexo_peer_count",
            "Number of connected peers with utreexo support",
            utreexo_peer_count.clone(),
        );

        registry.register(
            "block_interval",
            "Block interval in seconds",
            block_interval.clone(),
        );

        Self {
            registry,
            block_height,
            memory_usage,
            peer_count,
            avg_block_processing_time,
            message_times,
            utreexo_peer_count,
            difficulty,
            banned_peer_count,
            block_interval
        }
    }

    /// Gets how much memory is being used by the system in which Floresta is
    /// running on, not how much memory Floresta itself it's using.
    pub fn update_memory_usage(&self) {
        let mut system = System::new_all();
        system.refresh_memory();

        // get used memory in gigabytes                / KB    / MB    / GB
        let used_memory = system.used_memory() as f64 / 1024. / 1024. / 1024.;
        self.memory_usage.set(used_memory);
    }
}

impl Default for AppMetrics {
    fn default() -> Self {
        Self::new()
    }
}

// Singleton to share metrics across crates
static METRICS: OnceLock<AppMetrics> = OnceLock::new();
pub fn get_metrics() -> &'static AppMetrics {
    METRICS.get_or_init(AppMetrics::new)
}

async fn metrics_handler() -> String {
    let mut buffer = String::new();
    encode(&mut buffer, &get_metrics().registry).unwrap();

    buffer
}

pub async fn metrics_server(metrics_server_address: SocketAddr) {
    let app = Router::new().route("/", get(metrics_handler));
    let listener = tokio::net::TcpListener::bind(metrics_server_address)
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
