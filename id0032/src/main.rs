// <p>We shall say that an $n$-digit number is pandigital if it makes use of all the digits $1$ to $n$ exactly once; for example, the $5$-digit number, $15234$, is $1$ through $5$ pandigital.</p>
//
// <p>The product $7254$ is unusual, as the identity, $39 \times 186 = 7254$, containing multiplicand, multiplier, and product is $1$ through $9$ pandigital.</p>
//
// <p>Find the sum of all products whose multiplicand/multiplier/product identity can be written as a $1$ through $9$ pandigital.</p>
//
// <div class="note">HINT: Some products can be obtained in more than one way so be sure to only include it once in your sum.</div>

use std::collections::HashSet;

fn is_good_pair(x: u32, y: u32) -> bool {
    let z = x * y;
    let mut chars: Vec<char> = format!("{x}{y}{z}").chars().collect();
    chars.sort();
    let string: String = chars.into_iter().collect();
    string.as_str() == "123456789"
}

fn main() {
    // Find out a required condition for the problem.
    // Let x, y, z be the triple that satisfies the condition,
    // and let a, b, c be their number of digits respectively.
    // This means that:
    // - 10^(a - 1) <= x < 10^a
    // - 10^(b - 1) <= y < 10^b
    // - 10^(c - 1) <= z < 10^c
    // Now, from x*y = z,
    // - 10^(a + b - 2) <= x*y = z < 10^c
    // - 10^(c - 1) <= z = x*y < 10^(a + b)
    // are obtained. Hence,
    //   c - 1 < a + b <= c + 2
    // holds. By the way, (x, y, z) is pandigital, so
    //   a + b + c = 9
    // holds. From the above conditions, we obtain
    //   3.5 <= c < 5.
    // Then c = 4 because c is an integer. Moreover
    //   a + b = 5
    // satisfies. This is useful requirement condition.
    // If we suppose a < b without loss of generality, we get that
    //   (a, b) = (1, 4), (2, 3)
    // are all cases we have to consider.

    let mut products = HashSet::new();

    for n1 in 1..=9 {
        for n2 in (1..=9).filter(|n| ![n1].contains(n)) {
            for n3 in (1..=9).filter(|n| ![n1, n2].contains(n)) {
                for n4 in (1..=9).filter(|n| ![n1, n2, n3].contains(n)) {
                    for n5 in (1..=9).filter(|n| ![n1, n2, n3, n4].contains(n)) {
                        // ** Case 1: (a, b) = (1, 4) **
                        let x = n1;
                        let y = 1000 * n2 + 100 * n3 + 10 * n4 + n5;
                        if is_good_pair(x, y) {
                            products.insert(x * y);
                            println!("{} x {} = {}", x, y, x * y);
                        }

                        // ** Case 2: (a, b) = (2, 3) **
                        let x = 10 * n1 + n2;
                        let y = 100 * n3 + 10 * n4 + n5;
                        if is_good_pair(x, y) {
                            products.insert(x * y);
                            println!("{} x {} = {}", x, y, x * y);
                        }
                    }
                }
            }
        }
    }

    let total: u32 = products.into_iter().sum();
    println!("Total = {total}");
}
