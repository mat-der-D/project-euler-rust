// <p>In the United Kingdom the currency is made up of pound (£) and pence (p). There are eight coins in general circulation:</p>
// <blockquote>1p, 2p, 5p, 10p, 20p, 50p, £1 (100p), and £2 (200p).</blockquote>
// <p>It is possible to make £2 in the following way:</p>
// <blockquote>1×£1 + 1×50p + 2×20p + 1×5p + 1×2p + 3×1p</blockquote>
// <p>How many different ways can £2 be made using any number of coins?</p>

fn main() {
    let mut count = 0;
    let res = 200u32;
    for n1 in 0..=(res / 200) {
        let res = res - 200 * n1;
        for n2 in 0..=(res / 100) {
            let res = res - 100 * n2;
            for n3 in 0..=(res / 50) {
                let res = res - 50 * n3;
                for n4 in 0..=(res / 20) {
                    let res = res - 20 * n4;
                    for n5 in 0..=(res / 10) {
                        let res = res - 10 * n5;
                        for n6 in 0..=(res / 5) {
                            let res = res - 5 * n6;
                            count += res / 2 + 1;
                        }
                    }
                }
            }
        }
    }
    println!("{count}");
}
