use anyhow::Result;
use std::{fs::File, io::{BufRead, BufReader}};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use itertools::Itertools;

#[derive(Clone,Debug,EnumIter)]
enum Operation {
    Plus,
    Times,
}

impl Operation {
    fn compute(&self, a: u32, b: u32) -> u32 {
        match self {
            Operation::Times => a * b,
            Operation::Plus => a + b,
        }
    }
    fn generate_sequence(
        seq: Option<Vec<Operation>>,
        desired_length: usize
    ) -> Vec<Vec<Operation>> {
        let seq = seq.unwrap_or(Vec::new());
        if seq.len() == desired_length { return vec!(seq) }
        Operation::iter()
            .map(|op| {
                let mut new = seq.clone();
                new.push(op);
                Operation::generate_sequence(Some(new), desired_length)
            }).fold(Vec::new(), |mut acc, x| {
                acc.extend(x);
                acc
            })
    }
}

fn main() -> Result<()> {
    //let input = File::open("day01/input.txt")?;
    let input = File::open("day07/test.txt")?;
    let lines = BufReader::new(input)
        .lines()
        .map(|l| {
            l.unwrap()
                .replace(":", "")
                .split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        }).collect::<Vec<_>>();

    let part_a: u32 = lines.iter()
        .map(|l| true_sum(l))
        .sum();
    println!("Part a is {}", part_a);

    Ok(())
}

fn true_sum(line: &Vec<u32>) -> u32 {
    // This is inefficient since we could just slice rather than recalculate
    let combos = Operation::generate_sequence(None, line.len() - 1);

    let possible = combos.iter()
        .map(|seq| {
            // Will apply operations
            let r = line.iter()
                .skip(1)
                .tuples()
                .zip(seq)
                .map(|((a, b), op)| op.compute(*a, *b))
                .sum();
            // TODO issue is that I'm just summing between terms
            // maybe use fold?

            line[0] == r
        }).any(|x| x);

    (possible as u32) * line[0]
}

