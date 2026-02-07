use std::time::Instant;

fn main() {
    let now = Instant::now();
    let mut counter = 0;
    loop {
        println!("Hello, world!");
        counter += 1;
        if counter == 1000000 {
            break;
        }
        println!("Counter: {counter}");
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {elapsed:.2?}");
}
