use anyhow::Result;
use std::{fs::File, io::{BufRead, BufReader}};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Clone,Debug,EnumIter)]
enum Operation {
    Plus,
    Times,
}

impl Operation {
    fn compute(&self, a: u64, b: u64) -> u64 {
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
    let input = File::open("day07/input.txt")?;
    //let input = File::open("day07/test.txt")?;
    let lines = BufReader::new(input)
        .lines()
        .map(|l| {
            l.unwrap()
                .replace(":", "")
                .split_whitespace()
                .map(|n| {println!("{:?}", n); n.parse::<u64>().unwrap()})
                .collect::<Vec<_>>()
        }).collect::<Vec<_>>();

    let part_a: u64 = lines.iter()
        .map(|l| true_sum(l))
        .sum();
    println!("Part a is {}", part_a);

    Ok(())
}

fn true_sum(line: &Vec<u64>) -> u64 {
    // This is inefficient since we could just slice rather than recalculate
    let combos = Operation::generate_sequence(None, line.len() - 2);
    println!("{:?}", combos);
    let possible = combos.iter()
        .map(|seq| {
            println!("---------------");
            println!("{:?}", seq);
            println!("{:?}", line);
            // Will apply operations
            let r = line[0] == line.iter()
                .skip(2) // skip total and also first item
                .zip(seq)
                .fold(line[1], |acc, (x, op)| op.compute(acc, *x) );
            println!("{}", r);
            r
        }).any(|x| x);
    (possible as u64) * line[0]
}

