use sysinfo::{System, SystemExt, ProcessExt, Pid};
use std::io::{self, Write};

fn main() {
    let mut sys = System::new_all();
    sys.refresh_memory();
    println!("Initial Memory: Total: {} KB, Used: {} KB", sys.total_memory() / 1024, sys.used_memory() / 1024);

    if let Some(process) = sys.process(Pid::from_u32(std::process::id())) {
        println!("Process RSS: {} KB", process.memory() / 1024);
    } else {
        println!("Could not get process memory.");
    }
    io::stdout().flush().unwrap();

    // Simulate some memory allocation
    let mut vec = Vec::new();
    for i in 0..1_000_000 {
        vec.push(i);
    }

    sys.refresh_memory();
    if let Some(process) = sys.process(Pid::from_u32(std::process::id())) {
        println!("After allocation Memory: Total: {} KB, Used: {} KB, Process RSS: {} KB",
                 sys.total_memory() / 1024, sys.used_memory() / 1024, process.memory() / 1024);
    } else {
        println!("After allocation Memory: Total: {} KB, Used: {} KB", sys.total_memory() / 1024, sys.used_memory() / 1024);
    }
    io::stdout().flush().unwrap();
}
