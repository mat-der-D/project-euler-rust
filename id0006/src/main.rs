// <p>The sum of the squares of the first ten natural numbers is,</p>
// $$1^2 + 2^2 + ... + 10^2 = 385.$$
// <p>The square of the sum of the first ten natural numbers is,</p>
// $$(1 + 2 + ... + 10)^2 = 55^2 = 3025.$$
// <p>Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is $3025 - 385 = 2640$.</p>
// <p>Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.</p>

fn main() {
    // 1^2 + 2^2 + ... + n^2 = n*(n + 1)*(2n + 1)/6
    // (1 + 2 + ... + n)^2 = n^2 (n + 1)^2 / 4
    // Thus, the latter minus the former equals:
    // n^2 (n + 1)^2 /4 - n*(n + 1)*(2n + 1)/6
    // = { 3*n*(n + 1) - 2*(2n + 1) }*n*(n + 1)/12
    // = (3*n^2 -n -2)*n*(n + 1)/12

    let n = 100_u32;
    println!("{}", (3 * n * n - n - 2) * n * (n + 1) / 12);
}
