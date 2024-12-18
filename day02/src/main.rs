use std::{fs::File, io::{BufRead, BufReader}};

fn is_safe(line: &Vec<u32>) -> bool {
    let monotonic = line.iter().is_sorted() || line.iter().rev().is_sorted();
    let diff = line.windows(2)
        .map(|w| w[1] as i32 - w[0] as i32)
        .collect::<Vec<_>>();
    let in_range = diff.iter()
        .map(|d| (1..=3).contains(&d.abs()) )
        .all(|b| b);
    monotonic && in_range
}

fn main() -> anyhow::Result<()> {
    // Process inputs
    let input = File::open("day02/input.txt")?;
    let lines: Vec<Vec<u32>> = BufReader::new(input)
        .lines()
        .map(|line| {
            line.unwrap()
                .split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        }).collect();

    // Part one
    let count_part_one = lines.iter()
        .map(|line| {
            is_safe(line)
        }).filter(|x| *x)
    .count();
    println!("Safe count is {}", count_part_one);

    // Part two
    let count_part_two = lines.iter()
        .map(|line| {
            // Loop the indexes of a line
            (0..line.len()).map(|i| {
                // Filter out one value
                let filtered = line.iter()
                    .enumerate()
                    .filter(|(index, _)| *index != i)
                    .map(|(_, value)| *value)
                    .collect::<Vec<_>>();
                is_safe(&filtered)
            }).any(|x| x) // We care if any combo is safe
        }).filter(|x| *x)
    .count();
    println!("Safe count is {:?}", count_part_two);

    Ok(())
}
