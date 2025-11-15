// <p>$n!$ means $n \times (n - 1) \times \cdots \times 3 \times 2 \times 1$.</p>
// <p>For example, $10! = 10 \times 9 \times \cdots \times 3 \times 2 \times 1 = 3628800$,<br>and the sum of the digits in the number $10!$ is $3 + 6 + 2 + 8 + 8 + 0 + 0 = 27$.</p>
// <p>Find the sum of the digits in the number $100!$.</p>

#[derive(Debug, Clone)]
struct BigDecimal {
    digits: Vec<u8>,
}

impl BigDecimal {
    fn one() -> Self {
        Self { digits: vec![1] }
    }

    fn multiply(&mut self, num: u8) {
        let mut carry_up = 0u8;
        for d in self.digits.iter_mut() {
            let multiplied = (*d as u16) * (num as u16); // 9 * 99 > 255 = u8::MAX
            *d = carry_up % 10 + (multiplied % 10) as u8;
            carry_up = carry_up / 10 + (multiplied / 10) as u8;
        }
        while carry_up != 0 {
            self.digits.push(carry_up % 10);
            carry_up /= 10;
        }
    }

    fn sum_of_digits(&self) -> u128 {
        let mut sum = 0;
        for &d in self.digits.iter() {
            sum += d as u128;
        }
        sum
    }
}

fn main() {
    let mut factorial = BigDecimal::one();
    for n in 2..=100 {
        factorial.multiply(n);
    }
    println!("{}", factorial.sum_of_digits());
}
