use std::error::Error;
use std::fs;
use std::time::Instant;

fn read_input(file_path: &str) -> Result<Vec<Vec<u8>>, Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;

    let banks: Result<Vec<Vec<u8>>, Box<dyn Error>> = contents
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let bank: Result<Vec<u8>, _> = line
                .split("")
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<u8>())
                .collect();
            // let bank: Result<Vec<u8>, _> = line.split("").map(|i| i.parse::<u8>()).collect();
            bank.map_err(|e| e.into())
        })
        .collect();

    banks
}

fn first_challenge(file_path: &str) -> Result<u64, Box<dyn Error>> {
    let banks = read_input(file_path)?;

    let mut accumulator: u64 = 0;
    for bank in banks.iter() {
        let l = bank.len();

        let j = bank
            .iter()
            .enumerate()
            .fold((0 as u8, 0 as u8), |acc, (i, x)| {
                if *x > acc.0 && i != l - 1 {
                    (*x, 0)
                } else {
                    if *x > acc.1 { (acc.0, *x) } else { acc }
                }
            });
        accumulator += j.0 as u64 * 10 + j.1 as u64;
    }

    Ok(accumulator)
}

fn find_optimal_joltage(bank: &Vec<u8>, num: usize) -> u64 {
    let l = bank.len();
    // println!("bank_l, num: {:?}, {:?}", l, num);

    let optimal_batteries: Vec<u8> = vec![0; num];

    let j = bank
        .iter()
        .enumerate()
        .fold(optimal_batteries, |acc, (i, x)| {
            let mut new_acc = acc.clone();
            // println!("i, x: {:?}, {:?}", i, x);
            // println!("\tacc: {:?}", acc);
            for (j, battery) in acc.iter().enumerate() {
                // println!("\tj, battery: {:?}, {:?}", j, battery);
                let max_i = l - num + j + 1;
                // println!("\tmax_i: {:?}", max_i);
                if *x > *battery && i < max_i {
                    // println!("\t*x > *battery true!");
                    
                    new_acc[j] = *x;
                    for k in (j + 1)..num {
                        new_acc[k] = 0;
                    }
                    break;
                }
            }
            // println!("\tnew_acc: {:?}", new_acc);
            new_acc
        });

    // println!("j: {:?}", j);

    let a = j.iter().enumerate().fold(0, |acc, (i , x)| {
        let base: u64 = 10;
        let m = base.pow((num-1-i) as u32);

        acc + m * *x as u64

    });

    a
}

fn second_challenge(file_path: &str) -> Result<u64, Box<dyn Error>> {
    let banks = read_input(file_path)?;

    let mut accumulator: u64 = 0;
    for bank in banks.iter() {
        let optimal_joltage = find_optimal_joltage(bank, 12);
        accumulator += optimal_joltage;
        // println!("Optimal joltage: {:?}", optimal_joltage)
    }

    Ok(accumulator)
}

pub fn entry() {
    println!("=== Day 3 ===");

    let file_path_test = "data/day3_test.txt";
    let file_path = "data/day3.txt";
    let iterations = 100;

    println!("=== Challenge 1 ===");
    match first_challenge(file_path_test) {
        Ok(result) => println!("Test result: {}", result),
        Err(e) => eprintln!("Error in test: {}", e),
    }

    let start = Instant::now();
    for _ in 0..iterations {
        let _ = first_challenge(file_path);
    }
    let duration = start.elapsed();
    match first_challenge(file_path) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => eprintln!("Error: {}", e),
    }
    println!(
        "Average time on {:?} iterations: {:?}",
        iterations,
        duration / iterations
    );

    println!("=== Challenge 2 ===");
    match second_challenge(file_path_test) {
        Ok(result) => println!("Test result: {}", result),
        Err(e) => eprintln!("Error in test: {}", e),
    }

    let start = Instant::now();
    for _ in 0..iterations {
        let _ = second_challenge(file_path);
    }
    let duration = start.elapsed();
    match second_challenge(file_path) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => eprintln!("Error: {}", e),
    }
    println!(
        "Average time on {:?} iterations: {:?}",
        iterations,
        duration / iterations
    );
}
