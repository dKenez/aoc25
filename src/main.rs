mod challenges;

use challenges::{day1, day2, day3, day4, day5, day6, day7};

fn main() {
    let day = 0; // Set to 0 to run all, or specific day number

    if day == 0 {
        // Run all days
        println!("\n=== Running all days ===\n");
        day1::entry();
        day2::entry();
        day3::entry();
        day4::entry();
        day5::entry();
        day6::entry();
        day7::entry();
    } else {
        // Run specific day
        match day {
            1 => day1::entry(),
            2 => day2::entry(),
            3 => day3::entry(),
            4 => day4::entry(),
            5 => day5::entry(),
            6 => day6::entry(),
            7 => day7::entry(),
            _ => println!("Invalid day selected!: {}", day),
        };
    }
}
