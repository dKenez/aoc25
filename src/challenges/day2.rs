use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::num::ParseIntError;
use std::time::Instant;

fn read_input(file_path: &str) -> Result<Vec<(u64, u64)>, Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;

    let ranges: Result<Vec<(u64, u64)>, Box<dyn Error>> = contents
        .lines()
        .filter(|line| !line.is_empty())
        .next()
        .ok_or("No valid lines found")?
        .split(',')
        .filter(|line| !line.is_empty())
        .map(|pair| {
            let parts: Vec<&str> = pair.split("-").collect();

            if parts.len() != 2 {
                return Err(format!("Invalid range format: {}", pair).into());
            }

            let start = parts[0].parse::<u64>();

            match &start {
                Ok(_) => (),
                Err(e) => eprintln!("Start is not good: {:?}, {:?}", parts[0], e),
            };
            let end = parts[1].parse::<u64>();
            match &end {
                Ok(_) => (),
                Err(e) => eprintln!("End is not good: {:?}, {:?}", parts[1], e),
            };

            Ok((start?, end?))
        })
        .collect();

    ranges
}

fn get_range_bounds_by_repetition(
    range_start: u64,
    range_end: u64,
    repeat: usize,
) -> Result<(u64, u64), Box<dyn Error>> {
    let range_start_str = range_start.to_string();
    let range_end_str = range_end.to_string();

    let start_len = range_start_str.len();
    let end_len = range_end_str.len();

    let lower_bound: u64;
    if start_len % repeat == 0 {
        let part_size = start_len / repeat;

        let first = &range_start_str[..part_size].parse::<u64>()?;

        // let second = &range_start_str[part_size..2 * part_size].parse::<u64>()?;

        let rest: Result<Vec<u64>, ParseIntError> = (1..repeat)
            .map(|part| {
                let start = part * part_size;
                let end = (part + 1) * part_size;
                range_start_str[start..end].parse::<u64>()
            })
            .collect();

        let vec = rest?;

        lower_bound = vec
            .iter()
            .find(|&&n| first != &n)
            .map(|n| if first > n { *first } else { *first + 1 })
            .unwrap_or(*first);
    } else {
        let base: u64 = 10;
        lower_bound = base.pow((start_len / repeat) as u32);
    }

    let upper_bound: u64;
    if end_len % repeat == 0 {
        let part_size = end_len / repeat;

        let first = &range_end_str[..part_size].parse::<u64>()?;
        // let second = &range_end_str[part_size..2 * part_size].parse::<u64>()?;

        let rest: Result<Vec<u64>, ParseIntError> = (1..repeat)
            .map(|part| {
                let start = part * part_size;
                let end = (part + 1) * part_size;
                range_end_str[start..end].parse::<u64>()
            })
            .collect();

        let vec = rest?;
        
        upper_bound = vec
            .iter()
            .find(|&&n| first != &n)
            .map(|n| if first < n { *first } else { *first - 1 })
            .unwrap_or(*first);
    } else {
        let base: u64 = 10;
        upper_bound = base.pow((end_len / repeat) as u32) - 1;
    }
    // println!("Bounds: {:?}-{:?} r{:?}", lower_bound, upper_bound, repeat);
    Ok((lower_bound, upper_bound))
}

fn get_range_bounds(
    range_start: u64,
    range_end: u64,
) -> Result<Vec<(u64, u64, usize)>, Box<dyn Error>> {
    let range_end_str = range_end.to_string();

    let end_len = range_end_str.len();

    let min_repeat: usize = 2;
    let max_repeat = end_len;

    (min_repeat..=max_repeat)
        .map(|repeat| {
            let range_bounds = get_range_bounds_by_repetition(range_start, range_end, repeat)?;
            Ok((range_bounds.0, range_bounds.1, repeat))
        })
        .collect()
}

fn first_challenge(file_path: &str) -> Result<u64, Box<dyn Error>> {
    let ranges = read_input(file_path)?;

    let mut acc: u64 = 0;
    for range in ranges.iter() {
        let (lower_bound, upper_bound) = get_range_bounds_by_repetition(range.0, range.1, 2)?;
        // let upper_bound = define_upper_bound(range.1)?;
        for n in lower_bound..=upper_bound {
            let r = n.to_string().repeat(2).parse::<u64>()?;
            acc += r;
        }
    }

    Ok(acc)
}

fn second_challenge(file_path: &str) -> Result<u64, Box<dyn Error>> {
    let ranges = read_input(file_path)?;

    let mut acc: u64 = 0;

    let mut seen_numbers: HashSet<u64> = HashSet::new();
    for range in ranges.iter() {
        // println!("Range: {:?} {:?}", range.0, range.1);
        let range_bounds = get_range_bounds(range.0, range.1)?;

        for (range_start, range_end, repeat) in range_bounds.iter() {
            // println!("\tBounds: {:?}-{:?} r{:?}", range_start, range_end, repeat);
            for n in *range_start..=*range_end {
                let r = n.to_string().repeat(*repeat as usize).parse::<u64>()?;

                if !seen_numbers.contains(&r) {
                    seen_numbers.insert(r);
                    acc += r;
                    // println!("\t: {:?}", r);
                } else {
                    // println!("\talready found: {:?}", r);
                }
            }
        }
    }

    Ok(acc)
}

pub fn entry() {
    println!("=== Day 2 ===");

    let file_path_test = "data/day2_test.txt";
    let file_path = "data/day2.txt";
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
