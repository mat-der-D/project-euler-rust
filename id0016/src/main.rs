// <p>$2^{15} = 32768$ and the sum of its digits is $3 + 2 + 7 + 6 + 8 = 26$.</p>
// <p>What is the sum of the digits of the number $2^{1000}$?</p>

#[derive(Debug, Clone)]
struct BigDecimal {
    digits: Vec<u8>,
}

impl BigDecimal {
    fn one() -> Self {
        Self { digits: vec![1] }
    }

    fn twice(&mut self) {
        let mut carry_up = false;
        for d in self.digits.iter_mut() {
            let double_d = 2 * (*d);
            *d = double_d % 10;
            if carry_up {
                *d += 1;
            }
            carry_up = double_d >= 10;
        }
        if carry_up {
            self.digits.push(1);
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
    let mut num = BigDecimal::one();
    for _ in 0..1000 {
        num.twice();
    }
    println!("{}", num.sum_of_digits());
}
