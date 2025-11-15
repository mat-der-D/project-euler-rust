// <p>By starting at the top of the triangle below and moving to adjacent numbers on the row below, the maximum total from top to bottom is 23.</p>
// <p class="monospace center"><span class="red"><b>3</b></span><br><span class="red"><b>7</b></span> 4<br>
// 2 <span class="red"><b>4</b></span> 6<br>
// 8 5 <span class="red"><b>9</b></span> 3</p>
// <p>That is, 3 + 7 + 4 + 9 = 23.</p>
// <p>Find the maximum total from top to bottom in <a href="resources/documents/0067_triangle.txt">triangle.txt</a> (right click and 'Save Link/Target As...'), a 15K text file containing a triangle with one-hundred rows.</p>
// <p class="smaller"><b>NOTE:</b> This is a much more difficult version of <a href="problem=18">Problem 18</a>. It is not possible to try every route to solve this problem, as there are $2^{99}$ altogether! If you could check one trillion ($10^{12}$) routes every second it would take over twenty billion years to check them all. There is an efficient algorithm to solve it. ;o)</p>

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn parse_pyramid(path: &Path) -> Result<Vec<Vec<u32>>, anyhow::Error> {
    let mut pyramid = Vec::new();
    let fs = File::open(path)?;
    let reader = BufReader::new(fs);
    for line in reader.lines() {
        let mut pyramid_line = Vec::new();
        for num_str in line?.split_whitespace() {
            pyramid_line.push(num_str.parse()?);
        }
        pyramid.push(pyramid_line);
    }
    Ok(pyramid)
}

fn calc_max_total(pyramid: &Vec<Vec<u32>>) -> u32 {
    if pyramid.is_empty() {
        panic!();
    }
    let mut totals = pyramid[pyramid.len() - 1].clone();
    for irow in (0..(pyramid.len() - 1)).rev() {
        let mut next_totals = Vec::new();
        for (icol, &num) in (&pyramid[irow]).iter().enumerate() {
            let sum = num + totals[icol].max(totals[icol + 1]);
            next_totals.push(sum);
        }
        totals = next_totals;
    }
    assert!(totals.len() == 1);
    totals[0]
}

fn main() -> Result<(), anyhow::Error> {
    // Suppose that "cargo run" is executed at the project root
    let input_path = Path::new("src/0067_triangle.txt");
    let pyramid = parse_pyramid(&input_path)?;
    let max_total = calc_max_total(&pyramid);
    println!("{max_total}");
    Ok(())
}
