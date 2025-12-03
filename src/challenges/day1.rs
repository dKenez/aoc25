use std::error::Error;
use std::fs;

fn read_input(file_path: &str) -> Result<Vec<i32>, Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;

    let rotations: Result<Vec<i32>, Box<dyn Error>> = contents
        .lines()
        .filter(|line| !line.is_empty())
        .map(|x| {
            let mut chars = x.chars();
            let sign = match chars.next() {
                Some('R') => 1,
                Some('L') => -1,
                _ => return Err(format!("Invalid direction in line: {}", x).into()),
            };

            let value = chars.as_str().parse::<i32>()?;
            Ok(sign * value)
        })
        .collect();

    rotations
}

fn first_challenge(file_path: &str) -> Result<i32, Box<dyn Error>> {
    let rotations = read_input(file_path)?;

    let mut lock_state: i32 = 50;

    let mut zero_count = 0;

    for rotation in rotations.iter() {
        lock_state = (lock_state + rotation) % 100;

        if lock_state == 0 {
            zero_count += 1;
        }
    }

    Ok(zero_count)
}

fn second_challenge(file_path: &str) -> Result<i32, Box<dyn Error>> {
    let rotations = read_input(file_path)?;

    let mut lock_state: i32 = 50;

    let mut zero_count = 0;

    for rotation in rotations.iter() {
        let extra_zero_hits = (rotation / 100).abs();
        let relative_movement = rotation % 100;

        let relative_lock_state = lock_state + relative_movement;

        let went_over_zero =
            (lock_state != 0 && (relative_lock_state <= 0 || relative_lock_state >= 100)) as i32;

        lock_state = ((relative_lock_state % 100) + 100) % 100;

        zero_count += extra_zero_hits + went_over_zero;
    }

    Ok(zero_count)
}

pub fn entry() {
    println!("=== Day 1 ===");

    let file_path_test = "data/day1_test.txt";
    let file_path = "data/day1.txt";

    println!("=== Challenge 1 ===");
    match first_challenge(file_path_test) {
        Ok(result) => println!("Test result: {}", result),
        Err(e) => eprintln!("Error in test: {}", e),
    }

    match first_challenge(file_path) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => eprintln!("Error: {}", e),
    }

    println!("=== Challenge 2 ===");
    match second_challenge(file_path_test) {
        Ok(result) => println!("Test result: {}", result),
        Err(e) => eprintln!("Error in test: {}", e),
    }

    match second_challenge(file_path) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => eprintln!("Error: {}", e),
    }
}
