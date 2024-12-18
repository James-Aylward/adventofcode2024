use std::{fs::File, io::{BufRead, BufReader}};

fn main() -> anyhow::Result<()> {

    let input = File::open("day01/input.txt")?;
    let buffered = BufReader::new(input);

    let mut left: Vec<u32> = vec![];
    let mut right: Vec<u32> = vec![];

    for line in buffered.lines() {
        let split: Vec<_> = line?
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        left.push(split[0].parse().unwrap());
        right.push(split[1].parse().unwrap());
    }

    left.sort();
    right.sort();

    let distance: u32 =
        left.iter()
        .zip(right.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum::<u32>();

    println!("Distance is {}", distance);

    let similarity: u32 =
        left.iter()
        .map(|a| {
            a * right.iter().filter(|&b| b == a).count() as u32
        })
        .sum();

    println!("Similarity is {}", similarity);

    Ok(())
}
