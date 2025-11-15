// <p>Starting in the top left corner of a $2 \times 2$ grid, and only being able to move to the right and down, there are exactly $6$ routes to the bottom right corner.</p>
// <div class="center">
// <img src="resources/images/0015.png?1678992052" class="dark_img" alt=""></div>
// <p>How many such routes are there through a $20 \times 20$ grid?</p>

fn comb(n: u128, r: u128) -> u128 {
    if r == 0 {
        return 1;
    }
    if n < r {
        return 0;
    }
    comb(n - 1, r - 1) * n / r
}

fn main() {
    // The answer is the number of the permutations of 20 downs and 20 rights.
    // This equals to 40C20.
    println!("{}", comb(40, 20));
}
