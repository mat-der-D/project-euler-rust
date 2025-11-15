// <p>You are given the following information, but you may prefer to do some research for yourself.</p>
// <ul><li>1 Jan 1900 was a Monday.</li>
// <li>Thirty days has September,<br>
// April, June and November.<br>
// All the rest have thirty-one,<br>
// Saving February alone,<br>
// Which has twenty-eight, rain or shine.<br>
// And on leap years, twenty-nine.</li>
// <li>A leap year occurs on any year evenly divisible by 4, but not on a century unless it is divisible by 400.</li>
// </ul><p>How many Sundays fell on the first of the month during the twentieth century (1 Jan 1901 to 31 Dec 2000)?</p>

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DayOfWeek {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

impl DayOfWeek {
    const DAY_OF_WEEK_ARRAY: [Self; 7] = [
        Self::Sunday,
        Self::Monday,
        Self::Tuesday,
        Self::Wednesday,
        Self::Thursday,
        Self::Friday,
        Self::Saturday,
    ];

    fn add(&self, days: usize) -> Self {
        let current_index = Self::DAY_OF_WEEK_ARRAY
            .iter()
            .position(|dow| dow == self)
            .unwrap();
        Self::DAY_OF_WEEK_ARRAY[(current_index + days) % 7]
    }
}

fn is_leap_year(year: usize) -> bool {
    if year % 400 == 0 {
        return true;
    }
    if year % 100 == 0 {
        return false;
    }
    year % 4 == 0
}

fn count_days(year: usize, month: usize) -> usize {
    match month {
        4 | 6 | 9 | 11 => 30,
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => panic!("bad month"),
    }
}

fn main() {
    let mut count_sundays = 0u32;
    let mut day_of_week = DayOfWeek::Monday;
    for year in 1900..=2000 {
        for month in 1..=12 {
            let days_in_the_month = count_days(year, month);
            day_of_week = day_of_week.add(days_in_the_month);
            if year == 1900 && month <= 11 {
                continue;
            }
            if year == 2000 && month == 12 {
                continue;
            }
            if day_of_week == DayOfWeek::Sunday {
                count_sundays += 1;
            }
        }
    }
    println!("{count_sundays}");
}
