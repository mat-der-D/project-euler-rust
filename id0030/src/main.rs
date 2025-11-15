// <p>Surprisingly there are only three numbers that can be written as the sum of fourth powers of their digits:
// $$\begin{align}
// 1634 &amp;= 1^4 + 6^4 + 3^4 + 4^4\\
// 8208 &amp;= 8^4 + 2^4 + 0^4 + 8^4\\
// 9474 &amp;= 9^4 + 4^4 + 7^4 + 4^4
// \end{align}$$
// </p><p class="smaller">As $1 = 1^4$ is not a sum it is not included.</p>
// <p>The sum of these numbers is $1634 + 8208 + 9474 = 19316$.</p>
// <p>Find the sum of all the numbers that can be written as the sum of fifth powers of their digits.</p>

fn sum_of_powers_of_digits(mut num: u32, pow: u32) -> u32 {
    let mut sum = 0;
    while num != 0 {
        sum += (num % 10).pow(pow);
        num /= 10;
    }
    sum
}

fn is_good_number(num: u32) -> bool {
    num == sum_of_powers_of_digits(num, 5)
}

fn main() {
    // Let d be the number of digits of given number x.
    // Obviously, x >= 10^{d - 1}.
    // If x satisfies the condition of the problem,
    //   x <= 9^5 * d
    // must hold. Therefore
    //   10^{d - 1} <= 9^5 * d.
    // This violates for d >= 7, so d <= 6 is necessary. Hence
    //   1 < x < 10^6

    let mut total = 0;
    for x in 2..1_000_000 {
        if is_good_number(x) {
            println!("Found: {x}");
            total += x;
        }
    }
    println!("Total = {total}");
}
