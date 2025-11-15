// <p>Euler discovered the remarkable quadratic formula:</p>
// <p class="center">$n^2 + n + 41$</p>
// <p>It turns out that the formula will produce $40$ primes for the consecutive integer values $0 \le n \le 39$. However, when $n = 40, 40^2 + 40 + 41 = 40(40 + 1) + 41$ is divisible by $41$, and certainly when $n = 41, 41^2 + 41 + 41$ is clearly divisible by $41$.</p>
// <p>The incredible formula $n^2 - 79n + 1601$ was discovered, which produces $80$ primes for the consecutive values $0 \le n \le 79$. The product of the coefficients, $-79$ and $1601$, is $-126479$.</p>
// <p>Considering quadratics of the form:</p>
// <blockquote>
// $n^2 + an + b$, where $|a| &lt; 1000$ and $|b| \le 1000$<br><br><div>where $|n|$ is the modulus/absolute value of $n$<br>e.g. $|11| = 11$ and $|-4| = 4$</div>
// </blockquote>
// <p>Find the product of the coefficients, $a$ and $b$, for the quadratic expression that produces the maximum number of primes for consecutive values of $n$, starting with $n = 0$.</p>

// Sieve of Eratosthenes
const UPPER_BOUND: usize = 100_000;

const IS_PRIME: [bool; UPPER_BOUND] = calc_is_prime();

const fn calc_is_prime() -> [bool; UPPER_BOUND] {
    let mut is_prime = [true; UPPER_BOUND];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut p = 0;
    while p < UPPER_BOUND {
        if !is_prime[p] {
            p += 1;
            continue;
        }
        let mut mul = 2 * p;
        while mul < UPPER_BOUND {
            is_prime[mul] = false;
            mul += p;
        }
        p += 1;
    }
    is_prime
}

#[derive(Debug, Clone, Copy)]
struct Quadratic {
    a: i32,
    b: i32,
}

impl Quadratic {
    fn new(a: i32, b: i32) -> Self {
        Self { a, b }
    }

    fn calc(&self, n: i32) -> i32 {
        n * n + self.a * n + self.b
    }
}

fn calc_consecutive_prime_length(quad: &Quadratic) -> usize {
    for n in 0..UPPER_BOUND {
        let val = quad.calc(n as i32);
        if val < 0 {
            return n;
        }
        if !IS_PRIME[val as usize] {
            return n;
        }
    }
    panic!("UPPER_BOUND is too small");
}

fn main() {
    let mut a_at_max = 0;
    let mut b_at_max = 0;
    let mut max_prime_len = 0;
    for b in (1..=1000).filter(|&n| IS_PRIME[n]) {
        for a in (-999..=999).step_by(2) {
            let quad = Quadratic::new(a, b as i32);
            let prime_len = calc_consecutive_prime_length(&quad);
            if max_prime_len < prime_len {
                a_at_max = quad.a;
                b_at_max = quad.b;
                max_prime_len = prime_len;
            }
        }
    }
    println!("(a, b) = ({a_at_max}, {b_at_max})");
    println!("ab = {}", a_at_max * b_at_max);
    println!("max_prime_len = {max_prime_len}");
}
