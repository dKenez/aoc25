use std::error::Error;
use std::fs;
use std::ops::Add;
use std::time::Instant;

#[derive(Debug, Clone, Copy)]
enum State {
    // Empty,
    Source,
    Splitter,
    Beam(u64),
}

impl Add for State {
    type Output = State;

    fn add(self, other: State) -> State {
        match (self, other) {
            (State::Beam(a), State::Beam(b)) => State::Beam(a + b),
            _ => panic!("Cannot add non-Beam states"),
        }
    }
}

fn read_input(file_path: &str) -> Result<Vec<Vec<State>>, Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;

    let tachyon_manifold: Result<Vec<Vec<State>>, Box<dyn Error>> = contents
        .lines()
        .filter(|line| !line.is_empty())
        .map(|row| {
            row.chars()
                .map(|x| match x {
                    '.' => Ok(State::Beam(0)),
                    'S' => Ok(State::Source),
                    '^' => Ok(State::Splitter),
                    '|' => Ok(State::Beam(1)),
                    _ => Err(format!("Invalid operator: {}", x).into()),
                })
                .collect()
        })
        .collect();

    tachyon_manifold
}

fn _print_manifold(manifold: Vec<Vec<State>>) {
    manifold.iter().for_each(|row| {
        let c: String = row
            .iter()
            .map(|x| match x {
                State::Beam(0) => '.',
                State::Beam(_) => '|',
                State::Splitter => '^',
                State::Source => 'S',
            })
            .collect();
        println!("{}", c);
    })
}

fn simulate_manifold(manifold: Vec<Vec<State>>) -> Result<(Vec<Vec<State>>, u64), Box<dyn Error>> {
    let mut split_count = 0;
    let max_w = match manifold.first() {
        Some(result) => result.len(),
        None => return Err(format!("Error fetching first row!").into()),
    };

    let mut simulated_manifold: Vec<Vec<State>> = vec![];

    for (i, row) in manifold.iter().enumerate() {
        if i == 0 {
            simulated_manifold.push(row.clone());
        } else {
            let prev_row = match simulated_manifold.last() {
                Some(result) => result,
                None => return Err(format!("Error fetching last row!").into()),
            };


            let mut new_row: Vec<State> = row.clone();

            for (j, x) in row.iter().enumerate() {
                // let beam_prev = beam_indices.contains(&i);

                match x {
                    State::Beam(_) => match prev_row[j] {
                        State::Beam(level) => {
                            new_row[j] = new_row[j] + State::Beam(level);
                        }
                        State::Source => new_row[j] = State::Beam(1),
                        _ => {}
                    },
                    State::Splitter => match prev_row[j] {
                        State::Beam(0) => {}
                        State::Beam(level) => {
                            split_count += 1;
                            // println!("split on ({:?}, {:?})", i, j);
                            if j > 0 {
                                new_row[j - 1] = new_row[j - 1] + State::Beam(level)
                            }
                            if j < max_w - 1 {
                                new_row[j + 1] = new_row[j + 1] + State::Beam(level)
                            }
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }

            simulated_manifold.push(new_row);
        }
    }

    // println!("{:?}", simulated_manifold);

    Ok((simulated_manifold, split_count))
}

fn first_challenge(file_path: &str) -> Result<u64, Box<dyn Error>> {
    let content = read_input(file_path)?;

    let simulated_manifold = simulate_manifold(content)?;
    // print_manifold(simulated_manifold.0);

    Ok(simulated_manifold.1)
}

fn second_challenge(file_path: &str) -> Result<u64, Box<dyn Error>> {
    let content = read_input(file_path)?;

    let simulated_manifold = simulate_manifold(content)?;

    let last_row = match simulated_manifold.0.last() {
                Some(result) => result,
                None => return Err(format!("Error fetching last row!").into()),
            };

    let total_paths = last_row.iter().fold(0, |count, v| {
        let c = match v {
            State::Beam(level) => *level,
            _ => 0u64
        };

        count + c
    });

    Ok(total_paths)
}

pub fn entry() {
    println!("=== Day 7 ===");

    let file_path_test = "data/day7_test.txt";
    let file_path = "data/day7.txt";
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
