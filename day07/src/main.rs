use anyhow::Result;
use std::{fs::File, io::{BufRead, BufReader}};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

trait Operation {
    fn compute(&self, a: u64, b: u64) -> u64;
} 

#[derive(Clone,Debug,EnumIter)]
enum PartA {
    Plus,
    Times,
}

impl Operation for PartA {
    fn compute(&self, a: u64, b: u64) -> u64 {
        match self {
            Self::Times => a * b,
            Self::Plus => a + b,
        }
    }
}

#[derive(Clone,Debug,EnumIter)]
enum PartB {
    Plus,
    Times,
    Concatenate,
}

impl Operation for PartB {
    fn compute(&self, a: u64, b: u64) -> u64 {
        match self {
            Self::Times => a * b,
            Self::Plus => a + b,
            Self::Concatenate => format!("{}{}", a, b).parse::<u64>().unwrap()
        }
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
                .map(|n| n.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        }).collect::<Vec<_>>();

    let part_a: u64 = lines.iter()
        .map(|l| true_sum::<PartA>(l))
        .sum();
    println!("Part a is {}", part_a);

    let part_b: u64 = lines.iter()
        .map(|l| true_sum::<PartB>(l))
        .sum();
    println!("Part b is {}", part_b);

    Ok(())
}

fn generate_sequence<T: Operation + IntoEnumIterator + Clone>(
    seq: Option<Vec<T>>,
    desired_length: usize
) -> Vec<Vec<T>> {
    let seq = seq.unwrap_or(Vec::new());
    if seq.len() == desired_length { return vec!(seq) }
    T::iter()
        .map(|op| {
            let mut new = seq.clone();
            new.push(op);
            generate_sequence(Some(new), desired_length)
        }).fold(Vec::new(), |mut acc, x| {
            acc.extend(x);
            acc
        })

}

fn true_sum<T: Operation + IntoEnumIterator + Clone>(line: &Vec<u64>) -> u64 {
    // This is inefficient since we could just slice rather than recalculate
    let combos = generate_sequence::<T>(None, line.len() - 2);
    let possible = combos.iter()
        .map(|seq| {
            // Will apply operations
            line[0] == line.iter()
                .skip(2) // skip total and also first item
                .zip(seq)
                .fold(line[1], |acc, (x, op)| op.compute(acc, *x) )
        }).any(|x| x);
    (possible as u64) * line[0]
}

