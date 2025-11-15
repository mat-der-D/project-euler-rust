// <p>The following iterative sequence is defined for the set of positive integers:</p>
// <ul style="list-style-type:none;">
// <li>$n \to n/2$ ($n$ is even)</li>
// <li>$n \to 3n + 1$ ($n$ is odd)</li></ul>
// <p>Using the rule above and starting with $13$, we generate the following sequence:
// $$13 \to 40 \to 20 \to 10 \to 5 \to 16 \to 8 \to 4 \to 2 \to 1.$$</p>
// <p>It can be seen that this sequence (starting at $13$ and finishing at $1$) contains $10$ terms. Although it has not been proved yet (Collatz Problem), it is thought that all starting numbers finish at $1$.</p>
// <p>Which starting number, under one million, produces the longest chain?</p>
// <p class="note"><b>NOTE:</b> Once the chain starts the terms are allowed to go above one million.</p>

use std::collections::HashMap;

fn count_lengths(init: u64) -> Vec<(u64, usize)> {
    let mut series = vec![init];
    let mut num = init;
    while num > 1 {
        num = if num % 2 == 0 { num / 2 } else { 3 * num + 1 };
        series.push(num);
    }

    series
        .into_iter()
        .rev()
        .enumerate()
        .map(|(len, num)| (num, len))
        .collect()
}

fn update_len_map(init: u64, len_map: &mut HashMap<u64, usize>) {
    if len_map.contains_key(&init) {
        return;
    }
    let lens = count_lengths(init);
    len_map.extend(lens);
}

fn main() {
    let mut len_map = HashMap::new();
    for n in (1..=1_000_000).rev() {
        update_len_map(n, &mut len_map);
    }
    let longest = len_map.iter().max_by_key(|(_, v)| **v);
    println!("{:?}", longest);
}
