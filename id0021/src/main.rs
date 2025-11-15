// <p>Let $d(n)$ be defined as the sum of proper divisors of $n$ (numbers less than $n$ which divide evenly into $n$).<br>
// If $d(a) = b$ and $d(b) = a$, where $a \ne b$, then $a$ and $b$ are an amicable pair and each of $a$ and $b$ are called amicable numbers.</p>
// <p>For example, the proper divisors of $220$ are $1, 2, 4, 5, 10, 11, 20, 22, 44, 55$ and $110$; therefore $d(220) = 284$. The proper divisors of $284$ are $1, 2, 4, 71$ and $142$; so $d(284) = 220$.</p>
// <p>Evaluate the sum of all the amicable numbers under $10000$.</p>

use std::collections::HashSet;

fn sum_of_proper_divisors(num: u32) -> u32 {
    (1..num).filter(|n| num % n == 0).sum()
}

fn main() {
    const UPPER_BOUND: u32 = 10000;
    let mut not_checked: Vec<u32> = (1..=10000).collect();
    let mut amicable_nums = HashSet::new();
    let mut not_amicable_nums = HashSet::new();

    let mut prev_num = None;
    while let Some(mut num) = not_checked.pop() {
        loop {
            let next_num = sum_of_proper_divisors(num);
            not_checked.retain(|&n| n != next_num);

            if prev_num == Some(next_num) {
                // They are amicable numbers!
                amicable_nums.insert(num);
                amicable_nums.insert(next_num);
                break;
            }
            if next_num > UPPER_BOUND // out of range
                || num == next_num // perfect number
                || amicable_nums.contains(&next_num)
                || not_amicable_nums.contains(&next_num)
            {
                not_amicable_nums.insert(num);
                if let Some(prev_num_val) = prev_num {
                    not_amicable_nums.insert(prev_num_val);
                }
                break;
            }
            if let Some(prev_num_val) = prev_num {
                not_amicable_nums.insert(prev_num_val);
            }

            prev_num = Some(num);
            num = next_num;
        }
    }
    println!("{}", amicable_nums.into_iter().sum::<u32>());
}
