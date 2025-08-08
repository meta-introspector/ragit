use ragit_memory_monitor::MemoryMonitor;
use solfunmeme_metameme::engine::MetaMemeEngine;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let mut monitor = MemoryMonitor::new(true, None, None);

    monitor.capture_and_log_snapshot("Start of program");

    // Initialize MetaMemeEngine
    let mut engine = MetaMemeEngine::new();
    monitor.capture_and_log_snapshot("After MetaMemeEngine initialization");

    // Example usage of MetaMemeEngine (replace with actual crashing scenario if known)
    // For now, we'll just generate a simple poem to observe memory.
    for i in 0..1000 {
        let poem = engine.generate_poem("🌀🎭🧬").await?;
        monitor.process_unit();
        monitor.capture_and_log_snapshot(&format!("After generating poem {}", i));
        if i % 100 == 0 {
            println!("Generated poem {}: {:?}", i, poem);
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }

    monitor.print_final_report();

    Ok(())
}
