use monoxide_backend::battery::BatteryMonitor;
use monoxide_backend::CollectStats;

#[tokio::main]
async fn main() {
    let mut monitor = BatteryMonitor::new();
    let stats = monitor.collect_stats();

    println!("{:?}", stats);
}
