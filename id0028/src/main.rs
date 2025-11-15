// <p>Starting with the number $1$ and moving to the right in a clockwise direction a $5$ by $5$ spiral is formed as follows:</p>
// <p class="monospace center"><span class="red"><b>21</b></span> 22 23 24 <span class="red"><b>25</b></span><br>
// 20  <span class="red"><b>7</b></span>  8  <span class="red"><b>9</b></span> 10<br>
// 19  6  <span class="red"><b>1</b></span>  2 11<br>
// 18  <span class="red"><b>5</b></span>  4  <span class="red"><b>3</b></span> 12<br><span class="red"><b>17</b></span> 16 15 14 <span class="red"><b>13</b></span></p>
// <p>It can be verified that the sum of the numbers on the diagonals is $101$.</p>
// <p>What is the sum of the numbers on the diagonals in a $1001$ by $1001$ spiral formed in the same way?</p>

fn main() {
    // New corner numbers of n by n spiral are written as follows:
    // - upper right: n^2
    // - upper left: n^2 - n + 1
    // - lower left: n^2 - 2n + 2
    // - lower right: n^2 - 3n + 3
    // The sum of them is 4n^2 -6n +6

    let mut sum = 1;
    for n in (3..=1001).step_by(2) {
        sum += 4 * n * n - 6 * n + 6;
    }
    println!("{sum}");
}
