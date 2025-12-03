mod challenges;

use challenges::{day1, day2};

fn main() {
    let day = 2;

    match day {
        1 => day1::entry(),
        2 => day2::entry(),
        _ => println!("Invalid day selected!: {}", day),
    };
}
