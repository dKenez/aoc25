use std::error::Error;
use std::fs;
use std::time::Instant;

fn read_input(file_path: &str) -> Result<Vec<Vec<u8>>, Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;

    let roll_layout: Result<Vec<Vec<u8>>, Box<dyn Error>> = contents
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Ok(0),
                    '@' => Ok(1),
                    _ => Err(format!("Invalid char: {}", c).into()),
                })
                .collect()
        })
        .collect();

    roll_layout
}

fn zero_padding(matrix: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let w = matrix[0].len();

    let mut padded_matrix: Vec<Vec<u8>> = matrix
        .iter()
        .map(|row| {
            let mut new_row = row.clone();
            new_row.insert(0, 0);
            new_row.push(0);

            new_row
        })
        .collect();

    padded_matrix.insert(0, vec![0; w + 2]);
    padded_matrix.push(vec![0; w + 2]);
    // println!("padded_matrix: {:?}", padded_matrix);

    return padded_matrix;
}

fn count_adjacent(matrix: Vec<Vec<u8>>) -> u64 {
    matrix.iter().enumerate().fold(0, |acc, (i, row)| {
        if i == 0 || i == matrix.len() - 1 {
            acc
        } else {
            acc + row.iter().enumerate().fold(0, |row_acc, (j, v)| {
                if *v == 0 || j == 0 || j == row.len() - 1 {
                    row_acc
                } else {
                    let count_adjacent = matrix[i - 1][j - 1]
                        + matrix[i - 1][j]
                        + matrix[i - 1][j + 1]
                        + matrix[i][j - 1]
                        + matrix[i][j + 1]
                        + matrix[i + 1][j - 1]
                        + matrix[i + 1][j]
                        + matrix[i + 1][j + 1];
                    // println!("i, j : ({:?}, {:?})", i, j);
                    // println!("count_adjacent {:?}", count_adjacent);
                    // println!("count_adjacent {:?}", count_adjacent);

                    // println!("{:?} {:?} {:?}", matrix[i - 1][j - 1],matrix[i - 1][j],matrix[i - 1][j + 1]);
                    // println!("{:?} @ {:?}", matrix[i][j - 1], matrix[i][j + 1]);
                    // println!("{:?} {:?} {:?}", matrix[i + 1][j - 1], matrix[i + 1][j], matrix[i + 1][j + 1]);

                    if count_adjacent < 4 {
                        row_acc + 1
                    } else {
                        row_acc
                    }
                }
            })
        }
    })
}

fn is_removable(matrix: &Vec<Vec<u8>>, i: usize, j: usize) -> bool {
    if matrix[i][j] == 1 && i > 0 && i < matrix.len() && j > 0 && j < matrix[i].len() {
        let count_adjacent = matrix[i - 1][j - 1]
            + matrix[i - 1][j]
            + matrix[i - 1][j + 1]
            + matrix[i][j - 1]
            + matrix[i][j + 1]
            + matrix[i + 1][j - 1]
            + matrix[i + 1][j]
            + matrix[i + 1][j + 1];

        if count_adjacent < 4 { true } else { false }
    } else {
        false
    }
}

fn first_challenge(file_path: &str) -> Result<u64, Box<dyn Error>> {
    let content = read_input(file_path)?;

    let zp = zero_padding(content);
    let ca = count_adjacent(zp);

    Ok(ca)
}

fn second_challenge(file_path: &str) -> Result<u64, Box<dyn Error>> {
    let content = read_input(file_path)?;

    let zp = zero_padding(content);

    let mut eroded_matrix = zp.clone();

    let start_count = eroded_matrix.iter().fold(0, |acc, row| {
        acc + row.iter().fold(0, |row_acc, v| row_acc + *v as u64)
    });

    // println!("rolls at start {:?}", start_count);

    let mut curr_count = start_count;
    let mut prev_count = 0;

    while curr_count != prev_count {
        eroded_matrix = eroded_matrix
            .iter()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .map(|(j, v)| {
                        if is_removable(&eroded_matrix, i, j) {
                            0
                        } else {
                            *v
                        }
                    })
                    .collect()
            })
            .collect();

        prev_count = curr_count;
        curr_count = eroded_matrix.iter().fold(0, |acc, row| {
            acc + row.iter().fold(0, |row_acc, v| row_acc + *v as u64)
        });

        // println!("removed {:?} rolls", prev_count - curr_count);
    }

    Ok(start_count - curr_count)
}

pub fn entry() {
    println!("=== Day 4 ===");

    let file_path_test = "data/day4_test.txt";
    let file_path = "data/day4.txt";
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
