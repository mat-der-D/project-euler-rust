// <p>A permutation is an ordered arrangement of objects. For example, 3124 is one possible permutation of the digits 1, 2, 3 and 4. If all of the permutations are listed numerically or alphabetically, we call it lexicographic order. The lexicographic permutations of 0, 1 and 2 are:</p>
// <p class="center">012   021   102   120   201   210</p>
// <p>What is the millionth lexicographic permutation of the digits 0, 1, 2, 3, 4, 5, 6, 7, 8 and 9?</p>

const N: u32 = 1_000_000 - 1;
const FACTORIALS: [u32; 10] = {
    let mut factorials = [0; 10];
    let mut factorial = 1;
    factorials[0] = factorial;
    let mut i = 1;
    while i < 10 {
        factorial *= i;
        factorials[i as usize] = factorial;
        i += 1;
    }
    factorials
};

fn find_coords(mut num: u32) -> Vec<u32> {
    // Returns [a_9, a_8, ..., a_1]
    let mut coords = Vec::new();
    for &factorial in FACTORIALS.iter().rev().take(9) {
        coords.push(num / factorial);
        num %= factorial;
    }
    coords
}

fn main() {
    // Let N = 1,000,000 - 1. Find a_1, a_2, ..., a_9 that satisty
    //   N = a_9 * 9! + a_8 * 8! + ... + a_1 * 1!
    // where
    //   0 <= a_n <= n
    // for each n = 1, 2, ... 9. Then,
    // - First digit is the a_9-th element of [0, 1, 2, ..., 9]
    // - Second digit is the a_8-th element of the remainder.
    // - Similarly, all digits are defined.

    let coords = find_coords(N);
    let mut the_number = Vec::new();
    let mut digits: Vec<u32> = (0..=9).collect();
    for coord in coords {
        let digit = digits.remove(coord as usize);
        the_number.push(digit);
    }
    the_number.push(digits.remove(0));
    let the_number: String = the_number.into_iter().map(|n| n.to_string()).collect();
    println!("{}", the_number);
}
