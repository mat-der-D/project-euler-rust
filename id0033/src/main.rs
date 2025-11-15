// <p>The fraction $49/98$ is a curious fraction, as an inexperienced mathematician in attempting to simplify it may incorrectly believe that $49/98 = 4/8$, which is correct, is obtained by cancelling the $9$s.</p>
// <p>We shall consider fractions like, $30/50 = 3/5$, to be trivial examples.</p>
// <p>There are exactly four non-trivial examples of this type of fraction, less than one in value, and containing two digits in the numerator and denominator.</p>
// <p>If the product of these four fractions is given in its lowest common terms, find the value of the denominator.</p>

use std::ops::MulAssign;

#[derive(Debug, Clone, Copy)]
struct Rational {
    num: u32,
    den: u32,
}

impl Rational {
    fn new(num: u32, den: u32) -> Self {
        Self { num, den }
    }

    fn create_reduced_by_char(&self) -> Vec<Self> {
        let mut reduced = Vec::new();

        let num_chars: Vec<char> = self.num.to_string().chars().collect();
        let den_chars: Vec<char> = self.den.to_string().chars().collect();
        assert!(num_chars.len() == 2);
        assert!(den_chars.len() == 2);

        for num_pos in 0..=1 {
            if let Some(den_pos) = den_chars.iter().position(|c| *c == num_chars[num_pos]) {
                let mut new_num_chars = num_chars.clone();
                new_num_chars.remove(num_pos);
                let mut new_den_chars = den_chars.clone();
                new_den_chars.remove(den_pos);

                let rational = Self::new(
                    new_num_chars[0].to_string().parse().unwrap(),
                    new_den_chars[0].to_string().parse().unwrap(),
                );
                reduced.push(rational);
            }
        }

        reduced
    }

    fn reduce(&mut self) {
        let gcd = Self::calc_gcd(self.num, self.den);
        self.num /= gcd;
        self.den /= gcd;
    }

    fn calc_gcd(x: u32, y: u32) -> u32 {
        if y == 0 {
            return x;
        }
        if y == 1 {
            return 1;
        }
        Self::calc_gcd(y, x % y)
    }
}

impl PartialEq for Rational {
    fn eq(&self, other: &Self) -> bool {
        self.num * other.den == self.den * other.num
    }
}

impl Eq for Rational {}

fn is_good_rational(rational: &Rational) -> bool {
    let reduced = rational.create_reduced_by_char();
    reduced.into_iter().any(|r| r == *rational)
}

impl MulAssign for Rational {
    fn mul_assign(&mut self, rhs: Self) {
        self.num *= rhs.num;
        self.den *= rhs.den;
    }
}

impl std::fmt::Display for Rational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.num, self.den)
    }
}

fn main() {
    let mut product = Rational::new(1, 1);

    for num in (10..=99).filter(|n| n % 10 != 0) {
        for den in ((num + 1)..=99).filter(|n| n % 10 != 0) {
            let rational = Rational::new(num, den);
            if is_good_rational(&rational) {
                product *= rational;
                println!("{rational}");
            }
        }
    }

    product.reduce();
    println!("Product: {product}");
}
