// <p>In the United Kingdom the currency is made up of pound (£) and pence (p). There are eight coins in general circulation:</p>
// <blockquote>1p, 2p, 5p, 10p, 20p, 50p, £1 (100p), and £2 (200p).</blockquote>
// <p>It is possible to make £2 in the following way:</p>
// <blockquote>1×£1 + 1×50p + 2×20p + 1×5p + 1×2p + 3×1p</blockquote>
// <p>How many different ways can £2 be made using any number of coins?</p>

fn count_ways_to_pay(price: u32, coin_values: &[u32]) -> u32 {
    if coin_values.len() == 0 {
        return if price == 0 { 1 } else { 0 };
    }
    if coin_values.len() == 1 {
        return if price % coin_values[0] == 0 { 1 } else { 0 };
    }

    let first_coin_value = coin_values[0];
    let rest_coins = &coin_values[1..];
    let mut count = 0;
    for num in 0..=(price / first_coin_value) {
        let rest_price = price - first_coin_value * num;
        count += count_ways_to_pay(rest_price, rest_coins);
    }
    count
}

fn main() {
    let coin_values = [200, 100, 50, 20, 10, 5, 2, 1];
    let price = 200;
    let count = count_ways_to_pay(price, &coin_values);
    println!("{count}");
}
