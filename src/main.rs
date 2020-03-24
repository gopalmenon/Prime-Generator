use primes::Prime;

fn main() {
    let mut prime = Prime::new();
    for _ in 1..50 {
        prime.next();
    }
    println!("50th prime is {}", prime.next().unwrap());
}
