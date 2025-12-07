mod challenges;

use challenges::{
    day1,
    day2,
    day3,
    day4,
    day5,
    day6,
};

fn main() {
    let day = 5;

    match day {
        1 => day1::entry(),
        2 => day2::entry(),
        3 => day3::entry(),
        4 => day4::entry(),
        5 => day5::entry(),
        6 => day6::entry(),
        _ => println!("Invalid day selected!: {}", day),
    };
}
