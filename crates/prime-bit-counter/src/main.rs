use num_prime::nt_funcs::primes;
use std::time::Instant;
use sysinfo::System;

const MAX_BIT_SIZE: u32 = 24; // Set a safe upper limit for bit size

fn main() {
    let mut sys = System::new_all();

    println!("| Bit Size   | 2^n        | Total Grid Points | Prime Count | Prime/Grid Ratio | Time (ms) | Memory (MB) |");
    println!("|------------|------------|-------------------|-------------|------------------|-----------|-------------|");

    for n in 2..=MAX_BIT_SIZE {
        let start_time = Instant::now();
        sys.refresh_all();
        let initial_memory = sys.used_memory();

        let two_pow_n = 1u128 << n;
        let max_val = two_pow_n - 1;
        let mut total_grid_points = 0;

        let current_primes = primes(max_val as u64);
        let prime_count = current_primes.len();

        for &p in current_primes.iter() {
            if p == 0 { continue; } // Skip 0 if it somehow appears
            let p_f64 = p as f64;
            let max_val_f64 = max_val as f64;

            if max_val_f64 < p_f64 { break; } // Optimization: if max_val is smaller than prime, no power will fit

            let max_power = max_val_f64.log(p_f64).floor() as u32;
            if max_power > 0 {
                total_grid_points += max_power;
            }
        }

        let elapsed_time = start_time.elapsed().as_millis();
        sys.refresh_all();
        let final_memory = sys.used_memory();
        let memory_used_mb = final_memory.abs_diff(initial_memory) as f64 / (1024.0 * 1024.0);

        let prime_grid_ratio = if total_grid_points > 0 { prime_count as f64 / total_grid_points as f64 } else { 0.0 };

        println!("| {:<10} | {:<10} | {:<17} | {:<11} | {:<16.4} | {:<9} | {:<11.2} |",
                 n, two_pow_n, total_grid_points, prime_count, prime_grid_ratio, elapsed_time, memory_used_mb);
    }
    println!("|------------|------------|--------------------|-------------------|-------------|------------------|-----------|-------------|");
}
