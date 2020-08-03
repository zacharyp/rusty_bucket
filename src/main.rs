mod primes;
use std::time::SystemTime;

fn main() {
    let now = SystemTime::now();
    let bound: u64 = 100000000;
    let primes: u64 = primes::prime_number_count(bound);

    println!("There are {} primes under {}", primes, bound);
    println!("which took {} milliseconds to compute", now.elapsed().unwrap().as_millis());

    let now2 = SystemTime::now();
    let primes2: Vec<u64> = primes::sieve_primes(bound);

    println!("There are {} primes under {}", primes2.len(), bound);
    println!("which took {} milliseconds to compute", now2.elapsed().unwrap().as_millis());

}
