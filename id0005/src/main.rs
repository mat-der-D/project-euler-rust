// <p>$2520$ is the smallest number that can be divided by each of the numbers from $1$ to $10$ without any remainder.</p>
// <p>What is the smallest positive number that is <strong class="tooltip">evenly divisible<span class="tooltiptext">divisible with no remainder</span></strong> by all of the numbers from $1$ to $20$?</p>

fn main() {
    // The product of the max power of primes that are less than 20
    let answer: u32 = 16 * 9 * 5 * 7 * 11 * 13 * 17 * 19;
    println!("{answer}");
}
