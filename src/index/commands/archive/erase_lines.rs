pub fn erase_lines(n: usize) {
    for _ in 0..n {
        print!("\x1B[A\x1B[2K");
    }
}
