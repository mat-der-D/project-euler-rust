// <p>By listing the first six prime numbers: $2, 3, 5, 7, 11$, and $13$, we can see that the $6$th prime is $13$.</p>
// <p>What is the $10\,001$st prime number?</p>

fn is_prime(num: u64, known_primes: &[u64]) -> bool {
    known_primes.iter().all(|&p| num % p != 0)
}

fn main() {
    const TARGET_INDEX: usize = 10_001;
    let mut primes = Vec::with_capacity(TARGET_INDEX);
    primes.push(2);
    let mut p = 3;
    while primes.len() < TARGET_INDEX {
        if is_prime(p, &primes) {
            primes.push(p);
        }
        p += 2;
    }
    println!("{}", primes[TARGET_INDEX - 1]);
}
