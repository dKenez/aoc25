use std::error::Error;
use std::fs;
use std::iter::zip;
use std::num::ParseIntError;
// use std::string::ParseError;
use std::time::Instant;

#[derive(Debug)]
enum Operator {
    Add,
    Multiply,
}

fn read_input(file_path: &str) -> Result<(Vec<Vec<u64>>, Vec<Operator>), Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;

    let num_lines: usize = contents
        .lines()
        .filter(|line| !line.is_empty())
        .fold(0, |acc, _| acc + 1);

    let num_number_lines = num_lines - 1;
    let mut number_lines: Vec<Vec<u64>> = vec![];
    let mut operator_lines: Vec<Operator> = vec![];

    for (line_num, line) in contents.lines().filter(|line| !line.is_empty()).enumerate() {
        if line_num < num_number_lines {
            let l: Result<Vec<u64>, ParseIntError> = line
                .split(" ")
                .into_iter()
                .filter(|line| !line.is_empty())
                .map(|x| x.parse::<u64>())
                .collect();
            number_lines.push(l.unwrap());
        } else {
            let l: Result<Vec<Operator>, Box<dyn Error>> = line
                .split(" ")
                .into_iter()
                .filter(|line| !line.is_empty())
                .map(|x| match x {
                    "+" => Ok(Operator::Add),
                    "*" => Ok(Operator::Multiply),
                    _ => Err(format!("Invalid operator: {}", x).into()),
                })
                .collect();
            operator_lines = l.unwrap();
        }
    }

    let expected_len = operator_lines.len();
    assert!(
        number_lines.iter().all(|v| v.len() == expected_len),
        "Not all vectors have length {}",
        expected_len
    );

    let mut transposed_numbers = vec![vec![0; num_number_lines]; expected_len];

    for (i, number_line) in number_lines.iter().enumerate() {
        for (j, v) in number_line.iter().enumerate() {
            transposed_numbers[j][i] = *v;
        }
    }

    Ok((transposed_numbers, operator_lines))
}

fn read_input_2(file_path: &str) -> Result<(Vec<Vec<u64>>, Vec<Operator>), Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;

    let file_lines: Vec<&str> = contents
        .lines()
        .filter(|line| !line.is_empty())
        .map(|x| x)
        .collect();
    let num_lines = file_lines.len();

    let number_lines: Vec<Vec<&str>> = file_lines
        .get(..num_lines - 1)
        .unwrap()
        .iter()
        .map(|line| {
            line.split("")
                .filter(|line| !line.is_empty())
                .collect::<Vec<&str>>()
        })
        .collect();
    let operators = file_lines
        .get(num_lines - 1)
        .unwrap()
        .split(" ")
        .into_iter()
        .filter(|line| !line.is_empty())
        .map(|x| match x {
            "+" => Ok(Operator::Add),
            "*" => Ok(Operator::Multiply),
            _ => Err(format!("Invalid operator: {}", x).into()),
        })
        .collect::<Result<Vec<Operator>, Box<dyn Error>>>()
        .unwrap();

    let mut numbers: Vec<Vec<u64>> = vec![];

    let mut number_group: Vec<u64> = vec![];
    for (i, _) in number_lines[0].iter().enumerate() {
        let n = number_lines
            .iter()
            .map(|number_vec| {
                let candidate = number_vec[i];
                if candidate == " " { "" } else { candidate }
            })
            .collect::<String>()
            .parse::<u64>();

        match n {
            Ok(result) => number_group.push(result),
            Err(_) => {
                numbers.push(number_group);

                number_group = vec![];
            }
        }
    }

    numbers.push(number_group);

    // println!("numbers: {:?}", numbers);
    // println!("operators: {:?}", operators);

    Ok((numbers, operators))
}

fn first_challenge(file_path: &str) -> Result<u64, Box<dyn Error>> {
    let content = read_input(file_path)?;

    let mut grand_sum = 0;

    for (numbers, operator) in zip(content.0, content.1) {
        let result = match operator {
            Operator::Add => numbers.iter().fold(0, |acc, x| acc + x),
            Operator::Multiply => numbers.iter().fold(1, |acc, x| acc * x),
        };

        grand_sum += result;
    }

    Ok(grand_sum)
}

fn second_challenge(file_path: &str) -> Result<u64, Box<dyn Error>> {
    let content = read_input_2(file_path)?;

    let mut grand_sum = 0;

    for (numbers, operator) in zip(content.0, content.1) {
        let result = match operator {
            Operator::Add => numbers.iter().fold(0, |acc, x| acc + x),
            Operator::Multiply => numbers.iter().fold(1, |acc, x| acc * x),
        };

        grand_sum += result;
    }

    Ok(grand_sum)
}

pub fn entry() {
    println!("=== Day 6 ===");

    let file_path_test = "data/day6_test.txt";
    let file_path = "data/day6.txt";
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
