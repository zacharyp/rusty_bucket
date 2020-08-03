
pub fn sieve(bound: u64) -> Vec<bool> {
    if bound < 2 {
        return Vec::new();
    }
    let max_index: u64 = (bound - 1) / 2;
    // initialize sieve with all true
    let mut sieve: Vec<bool> = vec![true; (max_index + 1) as usize];

    // loop odd numbers up to square root
    let i_loop = (((bound as f64).sqrt()) as u64) / 2;
    for n in 0..i_loop {
        // check for prime
        if sieve[n as usize] {
            // unmark all odd multiples of the prime
            let prime: u64 = n * 2 + 3;
            let mut prime_loop = prime + prime * 2;
            while prime_loop <= bound {
                sieve[((prime_loop -3) / 2) as usize] = false;
                prime_loop += prime * 2;
            }
        }
    }

    sieve
}

pub fn sieve_primes(bound: u64) -> Vec<u64> {
    let s: Vec<bool> = sieve(bound);
    let max_index: u64 = (bound - 1) / 2;

    let mut result: Vec<u64> = vec![2];
    for ndx in 0..max_index {
        if s[ndx as usize] {
            result.push(ndx * 2 + 3)
        }
    }

    result
}


pub fn prime_number_count(bound: u64) -> u64 {
    sieve(bound).iter().fold(0u64, |acc, &current| acc + if current { 1 } else { 0 })
}