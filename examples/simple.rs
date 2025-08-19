use anyhow::Result;

use rerun::RecordingStreamBuilder;

use tracing_subscriber::{filter::LevelFilter, Registry};
use tracing_rerun::RerunLayer;
use tracing_subscriber::prelude::*;

fn main() -> Result<()> {
    // Create a Rerun recording (change to .spawn(), .save(), or .connect_tcp(...) as you prefer)
    let rec = RecordingStreamBuilder::new("my_app").serve_grpc()?;

    let rerun_layer = RerunLayer {
        rec: rec.clone(),
        path: "logs/tracing".into(),
    };

    // Install globally (try_init avoids panics if something already set a subscriber)
    let subscriber = Registry::default()
        .with(LevelFilter::INFO) // filter out low-level debug tracing (eg tokio executor)
        .with(tracing_subscriber::fmt::Layer::default()) // log to stdout
        .with(rerun_layer);

    tracing::subscriber::set_global_default(subscriber).expect("setting global default failed");

    // Example usage

    loop {
        tracing::error!(user_id = 42, "hello from tracing — goes to stdout and Rerun");
        tracing::info!(target: "db", op = "select", rows = 3, "query finished");
        tracing::warn!(target: "auth", "token expired");
        std::thread::sleep(std::time::Duration::from_millis(5000));
    }

}
