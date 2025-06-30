use std::time::Instant;

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn main() {
    let now: Instant = Instant::now();

    println!("Value: {}", fibonacci(47));

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
