use std::time::Instant;

fn main() {
    // start timer
    let start = Instant::now();

    // code to be timed
    for _ in 0..1000000 {
        // an example code
    }

    let duration = start.elapsed();

    println!("Execution Time: {:?}", duration);
}
