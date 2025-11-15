// <p>A unit fraction contains $1$ in the numerator. The decimal representation of the unit fractions with denominators $2$ to $10$ are given:</p>
// $$\begin{align}
// 1/2 &amp;= 0.5\\
// 1/3 &amp;=0.(3)\\
// 1/4 &amp;=0.25\\
// 1/5 &amp;= 0.2\\
// 1/6 &amp;= 0.1(6)\\
// 1/7 &amp;= 0.(142857)\\
// 1/8 &amp;= 0.125\\
// 1/9 &amp;= 0.(1)\\
// 1/10 &amp;= 0.1
// \end{align}$$
// <p>Where $0.1(6)$ means $0.166666\cdots$, and has a $1$-digit recurring cycle. It can be seen that $1/7$ has a $6$-digit recurring cycle.</p>
// <p>Find the value of $d \lt 1000$ for which $1/d$ contains the longest recurring cycle in its decimal fraction part.</p>

fn calc_period(denominator: u32) -> usize {
    let mut seen = Vec::new();
    let mut res = 1;
    loop {
        res = (10 * res) % denominator;
        if res == 0 {
            return 0;
        }
        if let Some(pos) = seen.iter().position(|&n| n == res) {
            return seen.len() - pos;
        } else {
            seen.push(res);
        }
    }
}

fn main() {
    let mut longest_n = 0;
    let mut longest_cycle = 0;
    for n in 1..1000 {
        let cycle_len = calc_period(n);
        if longest_cycle < cycle_len {
            longest_n = n;
            longest_cycle = cycle_len;
        }
    }
    println!("{longest_n}: {longest_cycle}");
}
