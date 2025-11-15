// <p>If the numbers $1$ to $5$ are written out in words: one, two, three, four, five, then there are $3 + 3 + 5 + 4 + 4 = 19$ letters used in total.</p>
// <p>If all the numbers from $1$ to $1000$ (one thousand) inclusive were written out in words, how many letters would be used? </p>
// <br><p class="note"><b>NOTE:</b> Do not count spaces or hyphens. For example, $342$ (three hundred and forty-two) contains $23$ letters and $115$ (one hundred and fifteen) contains $20$ letters. The use of "and" when writing out numbers is in compliance with British usage.</p>

fn count_less_than_20(num: u32) -> usize {
    match num {
        0 => 0,  // (say nothing)
        1 => 3,  // one
        2 => 3,  // two
        3 => 5,  // three
        4 => 4,  // four
        5 => 4,  // five
        6 => 3,  // six
        7 => 5,  // seven
        8 => 5,  // eight
        9 => 4,  // nine
        10 => 3, // ten
        11 => 6, // eleven
        12 => 6, // twelve
        13 => 8, // thirteen
        14 => 8, // fourteen
        15 => 7, // fifteen
        16 => 7, // sixteen
        17 => 9, // seventeen
        18 => 8, // eighteen
        19 => 8, // nineteen
        _ => panic!("out of range"),
    }
}

fn count_less_than_100(num: u32) -> usize {
    if num > 100 {
        panic!("out of range");
    }
    if num < 20 {
        return count_less_than_20(num);
    }
    let tens_place = match num / 10 {
        2 => 6, // twenty
        3 => 6, // thirty
        4 => 5, // forty
        5 => 5, // fifty
        6 => 5, // sixty
        7 => 7, // seventy
        8 => 6, // eighty
        9 => 6, // ninety
        _ => unreachable!(),
    };
    let ones_place = count_less_than_20(num % 10);
    tens_place + ones_place
}

fn count_less_than_1000(num: u32) -> usize {
    if num > 1000 {
        panic!("out of range");
    }

    const HUNDRED_COUNT: usize = 7; // hundred
    const AND_COUNT: usize = 3; // and

    let hundreds_place = count_less_than_100(num / 100);
    let ones_tens_place = count_less_than_100(num % 100);
    match (hundreds_place, ones_tens_place) {
        (0, 0) => 0,
        (0, _) => ones_tens_place,
        (_, 0) => hundreds_place + HUNDRED_COUNT,
        (_, _) => hundreds_place + HUNDRED_COUNT + AND_COUNT + ones_tens_place,
    }
}

fn main() {
    let mut total = 0;
    for n in 1..1000 {
        total += count_less_than_1000(n);
    }
    total += 11; // one thousand
    println!("{total}");
}
