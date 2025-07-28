use sysinfo::{System, Pid};
use std::io::{self, Write};

pub fn print_memory_usage(sys: &mut System, message: &str) {
    sys.refresh_memory();
    if let Some(process) = sys.process(Pid::from_u32(std::process::id())) {
        println!("Memory Usage ({}): Total: {} KB, Used: {} KB, Process RSS: {} KB",
                 message, sys.total_memory() / 1024, sys.used_memory() / 1024, process.memory() / 1024);
    } else {
        println!("Memory Usage ({}): Total: {} KB, Used: {} KB", message, sys.total_memory() / 1024, sys.used_memory() / 1024);
    }
    io::stdout().flush().unwrap();
}
