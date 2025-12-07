use std::error::Error;
use std::fs;
use std::time::Instant;

fn read_input(file_path: &str) -> Result<(Vec<(u64, u64)>, Vec<u64>), Box<dyn Error>> {
    let mut contents = fs::read_to_string(file_path)?;

    let breakpoint = match contents.find("\n\n") {
        Some(result) => result,
        None => return Err(format!("No breakpoint found!").into()),
    };

    // println!("breakpoint: {:?}", breakpoint);
    let ingredients_content = contents.split_off(breakpoint);
    let ranges_content = contents;

    // println!("ranges_content: {:?}", ranges_content);
    // println!("ingredients_content: {:?}", ingredients_content);

    let ranges = ranges_content
        .split("\n")
        .map(|range_str| {
            let nums = range_str
                .split("-")
                .map(|num| num.parse::<u64>())
                .collect::<Result<Vec<u64>, _>>()?;

            if nums.len() != 2 {
                return Err(format!("Expected 2 numbers, got {}", nums.len()).into());
            }

            Ok((nums[0], nums[1]))
        })
        .collect::<Result<Vec<(u64, u64)>, Box<dyn Error>>>()?;

    let ingredients = ingredients_content
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|ingredient_str| ingredient_str.parse::<u64>())
        .collect::<Result<Vec<u64>, _>>()?;

    // println!("ranges: {:?}", ranges);
    // println!("ingredients: {:?}", ingredients);

    Ok((ranges, ingredients))
}

fn first_challenge(file_path: &str) -> Result<u64, Box<dyn Error>> {
    let content = read_input(file_path)?;

    let ranges = content.0;
    let ingredients = content.1;

    let fresh_ingredients = ingredients.iter().fold(0, |count, ingredient| {
        for range in ranges.iter() {
            if *ingredient >= range.0 && *ingredient <= range.1 {
                return count + 1;
            }
        }

        count
    });

    Ok(fresh_ingredients)
}

fn merge_range_groups(range_groups: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let mut range_groups_sorted = range_groups.clone();
    range_groups_sorted.sort_by_key(|r| r.0);

    let new_range_groups =
        range_groups_sorted
            .iter()
            .fold(vec![], |range_groups: Vec<(u64, u64)>, curr_range| {
                let mut new_range_group = range_groups.clone();
                for (i, range_group) in range_groups.iter().enumerate() {
                    let curr_range_start_in_group =
                        range_group.0 <= curr_range.0 && curr_range.0 <= range_group.1;
                    let curr_range_end_in_group =
                        range_group.0 <= curr_range.1 && curr_range.1 <= range_group.1;

                    if curr_range_start_in_group || curr_range_end_in_group {
                        new_range_group[i].0 = if range_group.0 <= curr_range.0 {
                            range_group.0
                        } else {
                            curr_range.0
                        };
                        new_range_group[i].1 = if range_group.1 >= curr_range.1 {
                            range_group.1
                        } else {
                            curr_range.1
                        };

                        // println!("merged ({:?}-{:?}) with ({:?}-{:?}) => ({:?}-{:?})", curr_range.0, curr_range.1, range_group.0, range_group.1, new_range_group[i].0, new_range_group[i].1);
                        return new_range_group;
                    }
                }

                new_range_group.push(*curr_range);
                new_range_group
            });

    new_range_groups
}

fn second_challenge(file_path: &str) -> Result<u64, Box<dyn Error>> {
    let content = read_input(file_path)?;

    let ranges = content.0;
    // println!("range groups: {:?}", ranges);

    let start_len = ranges.len();
    // println!("start_len: {:?}", start_len);

    let mut new_ranges = merge_range_groups(ranges);

    let mut prev_len = start_len;
    let mut curr_len = new_ranges.len();

    while curr_len != prev_len {
        // println!("curr_len: {:?}", curr_len);
        // println!("range groups: {:?}", new_ranges);
        new_ranges = merge_range_groups(new_ranges);

        prev_len = curr_len;
        curr_len = new_ranges.len();
    }
    // println!("curr_len: {:?}", curr_len);
    new_ranges.sort_by(|a, b| a.0.cmp(&b.0));
    // println!("range groups: {:?}", new_ranges);

    let count = new_ranges.iter().fold(0, |count, range| {
        let range_count = (range.1 - range.0) + 1;
        let new_count = count + range_count;
        // println!("evaluating ({:?}-{:?}) -> {:?} count -> {:?} total count", range.0, range.1, range_count, new_count);
        new_count
    });

    Ok(count)
}

pub fn entry() {
    println!("=== Day 5 ===");

    let file_path_test = "data/day5_test.txt";
    let file_path = "data/day5.txt";
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

    let wrong_answers: Vec<u64> = vec![356439420655744];
    println!("wrong_answers: {:?}", wrong_answers);
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
