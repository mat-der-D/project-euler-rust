// <p>The four adjacent digits in the $1000$-digit number that have the greatest product are $9 \times 9 \times 8 \times 9 = 5832$.</p>
// <p class="monospace center">
// 73167176531330624919225119674426574742355349194934<br>
// 96983520312774506326239578318016984801869478851843<br>
// 85861560789112949495459501737958331952853208805511<br>
// 12540698747158523863050715693290963295227443043557<br>
// 66896648950445244523161731856403098711121722383113<br>
// 62229893423380308135336276614282806444486645238749<br>
// 30358907296290491560440772390713810515859307960866<br>
// 70172427121883998797908792274921901699720888093776<br>
// 65727333001053367881220235421809751254540594752243<br>
// 52584907711670556013604839586446706324415722155397<br>
// 53697817977846174064955149290862569321978468622482<br>
// 83972241375657056057490261407972968652414535100474<br>
// 82166370484403199890008895243450658541227588666881<br>
// 16427171479924442928230863465674813919123162824586<br>
// 17866458359124566529476545682848912883142607690042<br>
// 24219022671055626321111109370544217506941658960408<br>
// 07198403850962455444362981230987879927244284909188<br>
// 84580156166097919133875499200524063689912560717606<br>
// 05886116467109405077541002256983155200055935729725<br>
// 71636269561882670428252483600823257530420752963450<br></p>
// <p>Find the thirteen adjacent digits in the $1000$-digit number that have the greatest product. What is the value of this product?</p>

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn parse_input(path: &Path) -> Result<String, std::io::Error> {
    let fs = File::open(path)?;
    let reader = BufReader::new(fs);
    let mut out = String::new();
    for line in reader.lines() {
        let line = line?;
        out.push_str(&line);
    }
    Ok(out)
}

fn split_by_zero(num_series: &str) -> Vec<String> {
    let mut fragments = Vec::<String>::new();
    let mut s = String::new();
    for c in num_series.chars() {
        if c == '0' {
            if !s.is_empty() {
                fragments.push(s);
                s = String::new();
            }
            continue;
        }
        s.push(c);
    }
    if !s.is_empty() {
        fragments.push(s);
    }
    fragments
}

fn find_largest_prod(num_series: &str, digit_len: usize) -> Result<u64, anyhow::Error> {
    fn _parse_char(c: char) -> Result<u64, anyhow::Error> {
        Ok((c.to_string()).parse::<u64>()?)
    }

    // Initialization
    let head_iter = num_series.chars();
    let mut tail_iter = num_series.chars();
    let mut prod = 1;
    for _ in 0..digit_len {
        let c = tail_iter
            .next()
            .ok_or(anyhow::anyhow!("num_series is too short"))?;
        let num = _parse_char(c)?;
        prod *= num;
    }
    let mut largest_prod = prod;

    // Search
    for (head, tail) in head_iter.zip(tail_iter) {
        let head_num = _parse_char(head)?;
        let tail_num = _parse_char(tail)?;
        prod *= tail_num;
        prod /= head_num;
        largest_prod = largest_prod.max(prod);
    }
    Ok(largest_prod)
}

fn main() -> Result<(), anyhow::Error> {
    const DIGIT_LEN: usize = 13;
    // Suppose that "cargo run" is executed at the project root
    let input_path = Path::new("src/input.txt");
    let input_num_str = parse_input(&input_path)?;
    let num_fragments = split_by_zero(&input_num_str);

    let mut max_product = 0;
    for fragment in num_fragments {
        if let Ok(prod) = find_largest_prod(&fragment, DIGIT_LEN) {
            max_product = max_product.max(prod);
        }
    }
    println!("{max_product}");
    Ok(())
}
