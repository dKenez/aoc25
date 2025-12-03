mod challenges;

use challenges::{day1, day2, day3};

fn main() {
    let day = 3;

    match day {
        1 => day1::entry(),
        2 => day2::entry(),
        3 => day3::entry(),
        _ => println!("Invalid day selected!: {}", day),
    };
}
