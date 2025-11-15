// <p>A perfect number is a number for which the sum of its proper divisors is exactly equal to the number. For example, the sum of the proper divisors of $28$ would be $1 + 2 + 4 + 7 + 14 = 28$, which means that $28$ is a perfect number.</p>
// <p>A number $n$ is called deficient if the sum of its proper divisors is less than $n$ and it is called abundant if this sum exceeds $n$.</p>
//
// <p>As $12$ is the smallest abundant number, $1 + 2 + 3 + 4 + 6 = 16$, the smallest number that can be written as the sum of two abundant numbers is $24$. By mathematical analysis, it can be shown that all integers greater than $28123$ can be written as the sum of two abundant numbers. However, this upper limit cannot be reduced any further by analysis even though it is known that the greatest number that cannot be expressed as the sum of two abundant numbers is less than this limit.</p>
// <p>Find the sum of all the positive integers which cannot be written as the sum of two abundant numbers.</p>

use std::collections::HashSet;

fn sum_of_proper_divisors(num: u32) -> u32 {
    (1..num).filter(|n| num % n == 0).sum()
}

fn main() {
    const UPPER_BOUND: u32 = 28123;
    let abundant_nums: Vec<u32> = (12..=UPPER_BOUND)
        .filter(|&n| n < sum_of_proper_divisors(n))
        .collect();

    let mut good_nums = HashSet::new();
    for (i1, &n1) in abundant_nums.iter().enumerate() {
        for &n2 in abundant_nums.iter().skip(i1) {
            good_nums.insert(n1 + n2);
        }
    }

    let mut sum = 0;
    for n in 1..=UPPER_BOUND {
        if !good_nums.contains(&n) {
            sum += n;
        }
    }
    println!("{sum}");
}
