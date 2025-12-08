use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::num::ParseIntError;
use std::thread::AccessError;
use std::time::Instant;

fn read_input(file_path: &str) -> Result<Vec<(i64, i64, i64)>, Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;

    let vector_list: Result<Vec<(i64, i64, i64)>, _> = contents
        .lines()
        .filter(|line| !line.is_empty())
        .map(|row| {
            let parsed_vec = row
                .split(",")
                .map(|x| x.parse::<i64>())
                .collect::<Result<Vec<i64>, _>>()?;

            match parsed_vec.len() {
                3 => Ok((parsed_vec[0], parsed_vec[1], parsed_vec[2])),
                len => Err(format!("Invalid vector length: {}", len).into()),
            }
        })
        .collect();

    vector_list
}

fn merge_connections(connections: Vec<HashSet<usize>>) -> Vec<HashSet<usize>> {
    let mut new_connection_groups = connections.clone();

    let mut curr_len = new_connection_groups.len();
    let mut prev_len = 0;

    while curr_len != prev_len {
        new_connection_groups = new_connection_groups.iter().fold(
            vec![],
            |connection_groups: Vec<HashSet<usize>>, curr_set| {
                let mut merged_connection_groups = connection_groups.clone();

                let mut found_connection_group = false;

                for (i, set) in merged_connection_groups.iter().enumerate() {
                    let intersection: HashSet<_> = set.intersection(curr_set).collect();
                    if intersection.len() > 0 {
                        merged_connection_groups[i] = set.union(curr_set).copied().collect();
                        found_connection_group = true;
                        break;
                    }
                }

                if !found_connection_group {
                    merged_connection_groups.push(curr_set.clone());
                }

                merged_connection_groups
            },
        );

        prev_len = curr_len;
        curr_len = new_connection_groups.len();
    }

    new_connection_groups.sort_by_key(|s| -(s.len() as i32));
    new_connection_groups
}

// fn merge_connections_2(connections: Vec<HashSet<usize>>) -> HashSet<usize> {
//     let mut new_connection_groups = connections.clone();

//     let mut curr_len = new_connection_groups.len();
//     let mut prev_len = 0;

//     let mut last_connection = HashSet::new();

//     while curr_len != prev_len || curr_len == 1 {
//         new_connection_groups = new_connection_groups.iter().fold(
//             vec![],
//             |connection_groups: Vec<HashSet<usize>>, curr_set| {
//                 let mut merged_connection_groups = connection_groups.clone();

//                 let mut found_connection_group = false;

//                 for (i, set) in merged_connection_groups.iter().enumerate() {
//                     let intersection: HashSet<_> = set.intersection(curr_set).collect();
//                     if intersection.len() > 0 {
//                         merged_connection_groups[i] = set.union(curr_set).copied().collect();
//                         found_connection_group = true;
//                         break;
//                     }
//                 }

//                 if !found_connection_group {
//                     merged_connection_groups.push(curr_set.clone());
//                 }

//                 if merged_connection_groups.len() == 1 {
//                     last_connection = curr_set.clone();
//                 }
//                 merged_connection_groups
//             },
//         );

//         prev_len = curr_len;
//         curr_len = new_connection_groups.len();
//     }

//     // new_connection_groups.sort_by_key(|s| -(s.len() as i32));
//     last_connection
// }

fn first_challenge(file_path: &str, connections: usize) -> Result<u64, Box<dyn Error>> {
    let vector_list = read_input(file_path)?;

    // println!("vectors:\n{:?}", vector_list);
    let mut smallest_distances: Vec<(HashSet<usize>, f64)> =
        vec![(HashSet::new(), f64::INFINITY); connections];

    for (i, box_1) in vector_list.iter().enumerate() {
        for (j, box_2) in vector_list.iter().enumerate() {
            if i >= j {
                continue;
            }

            let distance = (((box_1.0 - box_2.0).pow(2)
                + (box_1.1 - box_2.1).pow(2)
                + (box_1.2 - box_2.2).pow(2)) as f64)
                .sqrt();

            let last = match smallest_distances.last() {
                Some(result) => result,
                None => {
                    return Err(
                        format!("Error fetching last item in smallest distances vector!").into(),
                    );
                }
            };

            if distance < last.1 {
                let set = HashSet::from([i, j]);
                smallest_distances[connections - 1] = (set, distance);

                smallest_distances.sort_by(|a, b| a.1.total_cmp(&b.1));
            }
        }
    }
    // println!("smallest_distances: {:?}", smallest_distances.len());
    // println!("smallest_distances: {:?}", smallest_distances);

    let merged_connection_groups = merge_connections(
        smallest_distances
            .iter()
            .map(|x| x.0.clone())
            .collect::<Vec<HashSet<usize>>>(),
    );
    // println!("merged_connection_groups: {:?}", merged_connection_groups);

    let product = merged_connection_groups[..3]
        .iter()
        .fold(1, |acc, s| acc * s.len());

    Ok(product as u64)
}

fn second_challenge(file_path: &str) -> Result<u64, Box<dyn Error>> {
    let vector_list = read_input(file_path)?;

    let len = vector_list.len();

    let mut smallest_distances: Vec<(HashSet<usize>, f64)> = vec![];

    for (i, box_1) in vector_list.iter().enumerate() {
        for (j, box_2) in vector_list.iter().enumerate() {
            if i >= j {
                continue;
            }

            let distance = (((box_1.0 - box_2.0).pow(2)
                + (box_1.1 - box_2.1).pow(2)
                + (box_1.2 - box_2.2).pow(2)) as f64)
                .sqrt();

            smallest_distances.push((HashSet::from([i, j]), distance));
        }
    }
    smallest_distances.sort_by(|a, b| a.1.total_cmp(&b.1));

    let mut acc_set: HashSet<usize> = HashSet::new();
    let mut acc_set_len = 0;
    for s in smallest_distances.iter() {
        acc_set = acc_set.union(&s.0).copied().collect();

        if acc_set.len() != acc_set_len {
            acc_set_len = acc_set.len();
            // println!("new connection: {:?} {:?}", acc_set.len(),  s.0);
        }

        if acc_set.len() == len {
            acc_set = s.0.clone();
            break;
        }
    }

    // println!("last_connection {:?}", acc_set);

    let product: u64 = acc_set
        .iter()
        .fold(1, |p, i| p * (vector_list[*i].0 as u64));

    Ok(product)
}

pub fn entry() {
    println!("=== Day 8 ===");

    let file_path_test = "data/day8_test.txt";
    let file_path = "data/day8.txt";
    let iterations = 100;

    println!("=== Challenge 1 ===");
    match first_challenge(file_path_test, 10) {
        Ok(result) => println!("Test result: {}", result),
        Err(e) => eprintln!("Error in test: {}", e),
    }

    let start = Instant::now();
    for _ in 0..iterations {
        let _ = first_challenge(file_path, 1000);
    }
    let duration = start.elapsed();
    match first_challenge(file_path, 1000) {
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
