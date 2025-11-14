// <p>The prime factors of $13195$ are $5, 7, 13$ and $29$.</p>
// <p>What is the largest prime factor of the number $600851475143$?</p>

const TARGET: u64 = 600_851_475_143;

fn main() {
    let mut num = TARGET;
    let mut divisor = 3;
    while divisor < num {
        while num % divisor == 0 {
            num /= divisor;
        }
        divisor += 2;
    }
    println!("{num}");
}
