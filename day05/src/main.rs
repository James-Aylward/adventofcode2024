use anyhow::Result;
use itertools::Itertools;
use std::{fs::File, io::{BufRead, BufReader}};

fn main() -> Result<()> {
    let input = File::open("day05/input.txt")?;
    let mut rules: Vec<(u32, u32)> = Vec::new();
    let mut updates: Vec<Vec<u32>> = Vec::new();
    let mut finished_rules = false;

    for line in BufReader::new(input).lines() {
        let line = line?;
        if line.trim().is_empty() {
            finished_rules = true;
        } else if finished_rules {
            let line = line.split(',')
                .map(|s| s.parse::<u32>().unwrap() )
                .collect::<Vec<_>>();
            updates.push(line);
        } else {
            let line: (u32, u32) = line.split('|')
                .map(|s| s.parse::<u32>().unwrap() )
                .next_tuple()
                .unwrap();
            rules.push(line);
        }
    }

    Ok(())
}
