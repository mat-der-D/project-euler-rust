// <p>The sum of the primes below $10$ is $2 + 3 + 5 + 7 = 17$.</p>
// <p>Find the sum of all the primes below two million.</p>

fn main() {
    // Sieve of Eratosthenes
    const UPPER_BOUND: usize = 2_000_000;
    let mut is_prime = [true; UPPER_BOUND + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    for p in 0..=UPPER_BOUND {
        if !is_prime[p] {
            continue;
        }
        for mul in ((2 * p)..=UPPER_BOUND).step_by(p) {
            is_prime[mul] = false;
        }
    }

    let mut sum = 0;
    for (p, &is_p) in is_prime.iter().enumerate() {
        if !is_p {
            continue;
        }
        sum += p;
    }
    println!("{sum}");
}
