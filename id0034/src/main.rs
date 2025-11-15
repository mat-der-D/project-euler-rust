// <p>$145$ is a curious number, as $1! + 4! + 5! = 1 + 24 + 120 = 145$.</p>
// <p>Find the sum of all numbers which are equal to the sum of the factorial of their digits.</p>
// <p class="smaller">Note: As $1! = 1$ and $2! = 2$ are not sums they are not included.</p>

const FACTORIALS: [usize; 10] = {
    let mut factorials = [1; 10];
    let mut factorial = 1;
    let mut n = 2;
    while n < 10 {
        factorial *= n;
        factorials[n] = factorial;
        n += 1;
    }
    factorials
};

fn calc_sum_of_digit_factorials(mut num: usize) -> usize {
    let mut sum = 0;
    while num > 0 {
        sum += FACTORIALS[num % 10];
        num /= 10;
    }
    sum
}

fn main() {
    // The number of digits d must satisfy the inequality:
    //   10^(d - 1) < 9! * d,
    // which only holds when d <= 8.

    let mut total = 0;
    for num in 10..100_000_000 {
        if num == calc_sum_of_digit_factorials(num) {
            total += num;
            println!("{num}");
        }
    }
    println!("Total = {total}");
}
