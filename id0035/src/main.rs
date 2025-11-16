// <p>The number, $197$, is called a circular prime because all rotations of the digits: $197$, $971$, and $719$, are themselves prime.</p>
// <p>There are thirteen such primes below $100$: $2, 3, 5, 7, 11, 13, 17, 31, 37, 71, 73, 79$, and $97$.</p>
// <p>How many circular primes are there below one million?</p>

use std::collections::{HashSet, VecDeque};

fn gather_primes(upper_bound: usize) -> HashSet<u32> {
    let mut is_prime: Vec<bool> = [true].into_iter().cycle().take(upper_bound + 1).collect();
    is_prime[0] = false;
    is_prime[1] = false;
    for p in 2..is_prime.len() {
        if !is_prime[p] {
            continue;
        }
        for mul in ((2 * p)..is_prime.len()).step_by(p) {
            is_prime[mul] = false;
        }
    }
    is_prime
        .into_iter()
        .enumerate()
        .filter_map(|(i, b)| if b { Some(i as u32) } else { None })
        .collect()
}

fn is_circular_prime(num: u32, primes: &HashSet<u32>) -> bool {
    if num == 0 {
        return false;
    }
    let mut num_deque = num_to_vecdeque(num);
    for _ in 0..num_deque.len() {
        num_deque.rotate_left(1);
        let rotated = vecdeque_to_num(num_deque.clone());
        if !primes.contains(&rotated) {
            return false;
        }
    }
    true
}

fn num_to_vecdeque(mut num: u32) -> VecDeque<u32> {
    let mut vecdeque = VecDeque::new();
    while num > 0 {
        vecdeque.push_front(num % 10);
        num /= 10;
    }
    vecdeque
}

fn vecdeque_to_num(mut vecdeque: VecDeque<u32>) -> u32 {
    let mut num = 0;
    while let Some(x) = vecdeque.pop_back() {
        num = 10 * num + x;
    }
    num
}

fn main() {
    let primes = gather_primes(1_000_000);
    let mut count = 0;
    for n in 1..1_000_000 {
        if is_circular_prime(n, &primes) {
            count += 1;
        }
    }
    println!("{count}");
}
