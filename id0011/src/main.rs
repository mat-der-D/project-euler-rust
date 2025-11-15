// <p>In the $20 \times 20$ grid below, four numbers along a diagonal line have been marked in red.</p>
// <p class="monospace center">
// 08 02 22 97 38 15 00 40 00 75 04 05 07 78 52 12 50 77 91 08<br>
// 49 49 99 40 17 81 18 57 60 87 17 40 98 43 69 48 04 56 62 00<br>
// 81 49 31 73 55 79 14 29 93 71 40 67 53 88 30 03 49 13 36 65<br>
// 52 70 95 23 04 60 11 42 69 24 68 56 01 32 56 71 37 02 36 91<br>
// 22 31 16 71 51 67 63 89 41 92 36 54 22 40 40 28 66 33 13 80<br>
// 24 47 32 60 99 03 45 02 44 75 33 53 78 36 84 20 35 17 12 50<br>
// 32 98 81 28 64 23 67 10 <span class="red"><b>26</b></span> 38 40 67 59 54 70 66 18 38 64 70<br>
// 67 26 20 68 02 62 12 20 95 <span class="red"><b>63</b></span> 94 39 63 08 40 91 66 49 94 21<br>
// 24 55 58 05 66 73 99 26 97 17 <span class="red"><b>78</b></span> 78 96 83 14 88 34 89 63 72<br>
// 21 36 23 09 75 00 76 44 20 45 35 <span class="red"><b>14</b></span> 00 61 33 97 34 31 33 95<br>
// 78 17 53 28 22 75 31 67 15 94 03 80 04 62 16 14 09 53 56 92<br>
// 16 39 05 42 96 35 31 47 55 58 88 24 00 17 54 24 36 29 85 57<br>
// 86 56 00 48 35 71 89 07 05 44 44 37 44 60 21 58 51 54 17 58<br>
// 19 80 81 68 05 94 47 69 28 73 92 13 86 52 17 77 04 89 55 40<br>
// 04 52 08 83 97 35 99 16 07 97 57 32 16 26 26 79 33 27 98 66<br>
// 88 36 68 87 57 62 20 72 03 46 33 67 46 55 12 32 63 93 53 69<br>
// 04 42 16 73 38 25 39 11 24 94 72 18 08 46 29 32 40 62 76 36<br>
// 20 69 36 41 72 30 23 88 34 62 99 69 82 67 59 85 74 04 36 16<br>
// 20 73 35 29 78 31 90 01 74 31 49 71 48 86 81 16 23 57 05 54<br>
// 01 70 54 71 83 51 54 69 16 92 33 48 61 43 52 01 89 19 67 48<br></p>
// <p>The product of these numbers is $26 \times 63 \times 78 \times 14 = 1788696$.</p>
// <p>What is the greatest product of four adjacent numbers in the same direction (up, down, left, right, or diagonally) in the $20 \times 20$ grid?</p>

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

const ARRAY_SIZE: usize = 20;
type MyArray = [[u32; ARRAY_SIZE]; ARRAY_SIZE];

fn can_calc_right(idx1: usize, idx2: usize) -> bool {
    idx1 < ARRAY_SIZE - 4 && idx2 < ARRAY_SIZE
}

fn calc_prod_right(idx1: usize, idx2: usize, nums: &MyArray) -> u32 {
    let mut prod = 1;
    for i in 0..4 {
        prod *= nums[idx1 + i][idx2];
    }
    prod
}

fn can_calc_right_down(idx1: usize, idx2: usize) -> bool {
    idx1 < ARRAY_SIZE - 4 && idx2 < ARRAY_SIZE - 4
}

fn calc_prod_right_down(idx1: usize, idx2: usize, nums: &MyArray) -> u32 {
    let mut prod = 1;
    for i in 0..4 {
        prod *= nums[idx1 + i][idx2 + i];
    }
    prod
}

fn can_calc_down(idx1: usize, idx2: usize) -> bool {
    idx1 < ARRAY_SIZE && idx2 < ARRAY_SIZE - 4
}

fn calc_prod_down(idx1: usize, idx2: usize, nums: &MyArray) -> u32 {
    let mut prod = 1;
    for i in 0..4 {
        prod *= nums[idx1][idx2 + i];
    }
    prod
}

fn can_calc_left_down(idx1: usize, idx2: usize) -> bool {
    idx1 >= 3 && idx1 < ARRAY_SIZE && idx2 < ARRAY_SIZE - 4
}

fn calc_prod_left_down(idx1: usize, idx2: usize, nums: &MyArray) -> u32 {
    let mut prod = 1;
    for i in 0..4 {
        prod *= nums[idx1 - i][idx2 + i];
    }
    prod
}

fn find_max_prod(nums: &MyArray) -> u32 {
    let mut max_prod = 1;
    for idx1 in 0..ARRAY_SIZE {
        for idx2 in 0..ARRAY_SIZE {
            if can_calc_right(idx1, idx2) {
                max_prod = max_prod.max(calc_prod_right(idx1, idx2, nums));
            }
            if can_calc_right_down(idx1, idx2) {
                max_prod = max_prod.max(calc_prod_right_down(idx1, idx2, nums));
            }
            if can_calc_down(idx1, idx2) {
                max_prod = max_prod.max(calc_prod_down(idx1, idx2, nums));
            }
            if can_calc_left_down(idx1, idx2) {
                max_prod = max_prod.max(calc_prod_left_down(idx1, idx2, nums));
            }
        }
    }
    max_prod
}

fn parse_array(path: &Path) -> Result<MyArray, anyhow::Error> {
    let mut nums = [[0; ARRAY_SIZE]; ARRAY_SIZE];

    let fs = File::open(path)?;
    let reader = BufReader::new(fs);
    for (irow, line) in reader.lines().enumerate() {
        let line = line?;
        for (icol, num_str) in line.split_whitespace().enumerate() {
            nums[irow][icol] = num_str.parse()?;
        }
    }
    Ok(nums)
}

fn main() -> Result<(), anyhow::Error> {
    // Suppose that "cargo run" is executed at the project root
    let path = Path::new("src/input.txt");
    let nums = parse_array(path)?;
    let max_prod = find_max_prod(&nums);
    println!("{max_prod}");
    Ok(())
}
