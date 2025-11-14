// <p>A palindromic number reads the same both ways. The largest palindrome made from the product of two $2$-digit numbers is $9009 = 91 \times 99$.</p>
// <p>Find the largest palindrome made from the product of two $3$-digit numbers.</p>

fn reverse(mut num: u32) -> u32 {
    let mut out = 0;
    while num > 0 {
        out *= 10;
        out += num % 10;
        num /= 10;
    }
    out
}

fn is_palindromic(num: u32) -> bool {
    num == reverse(num)
}

fn main() {
    let mut result = 0u32;
    for x in (100..=999).rev() {
        for y in (x..=999).rev() {
            let prod = x * y;
            if prod < result {
                continue;
            }
            if is_palindromic(prod) {
                result = prod;
            }
        }
    }
    println!("{result}");
}
