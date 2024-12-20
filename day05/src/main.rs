use anyhow::Result;
use itertools::Itertools;
use std::{fs::File, io::{BufRead, BufReader}};

fn main() -> Result<()> {
    let input = File::open("day05/input.txt")?;
    //let input = File::open("day05/test.txt")?;
    let mut rules: Vec<(u32, u32)> = Vec::new();
    let mut updates: Vec<Vec<u32>> = Vec::new();
    let mut finished_rules = false;

    for line in BufReader::new(input).lines() {
        let line = line?;
        if line.trim().is_empty() {
            finished_rules = true;
        } else if finished_rules {
            let line = line.split(',')
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            updates.push(line);
        } else {
            let line: (u32, u32) = line.split('|')
                .map(|s| s.parse::<u32>().unwrap())
                .next_tuple()
                .unwrap();
            rules.push(line);
        }
    }

    let part_a: u32 = updates.iter()
        .filter(|x| abides_rules(&x, &rules))
        .fold(0, |acc, x| acc + x[x.len()/2]);
    println!("Part a is {}", part_a);

    let part_b: u32 = updates.iter_mut()
        .filter(|x| !abides_rules(&x, &rules))
        .map(|x| fix(x.to_vec(), &rules))
        .fold(0, |acc, x| acc + x[x.len()/2]);

    println!("Part b is {}", part_b);

    Ok(())
}


// Repeated code in abides_rules and fix but CBA
fn abides_rules(update: &Vec<u32>, rules: &Vec<(u32,u32)>) -> bool {
    for rule in rules {
        let left = update.iter().position(|x| *x == rule.0);
        let right = update.iter().position(|x| *x == rule.1);

        if let (Some(left), Some(right)) = (left, right) {
            if right < left {
                return false;
            }
        }
    }
    true
}

// Repeated code in abides_rules and fix but CBA
fn fix(mut update: Vec<u32>, rules: &Vec<(u32, u32)>) -> Vec<u32> {
    let mut swapped = true;
    while swapped {
        swapped = false;
        for rule in rules {
            let left = update.iter().position(|x| *x == rule.0);
            let right = update.iter().position(|x| *x == rule.1);

            if let (Some(left), Some(right)) = (left, right) {
                if right < left {
                    update.swap(left, right);
                    swapped = true;
                }
            }
        }
    }
    update
}
