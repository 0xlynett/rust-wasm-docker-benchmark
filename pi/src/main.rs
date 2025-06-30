use rustedbytes_pi::compute_pi;
use std::time::Instant;

fn main() {
    let now = Instant::now();

    std::hint::black_box(compute_pi(3_141_592));

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
