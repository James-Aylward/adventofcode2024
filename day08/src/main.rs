use anyhow::Result;
use std::collections::HashMap;
use std::ops::{Add, Sub};
use std::{fs::File, io::{BufRead, BufReader}};
use itertools::Itertools;

type GridData = HashMap<char, Vec<Point>>;

#[derive(PartialEq,Copy,Clone,Hash,Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Point {
    type Output = Point;
    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

fn main() -> Result<()> {

    //let (grid, x, y) = load_file("day08/test.txt")?;
    let (grid, x, y) = load_file("day08/input.txt")?;
    
    let part_a = grid.values()
        .map(|v| compute_spots(v))
        .fold(Vec::new(), |mut acc, x| {
            acc.extend(x);
            acc
        }).iter()
        .unique()
        .filter(|p| {
            p.x >= 0 && p.y >= 0 && p.x < x && p.y < y
        }).count();

    println!("{}", part_a);
        
    Ok(())
}

fn compute_spots(points: &Vec<Point>) -> Vec<Point> {
    let mut result = Vec::new();

    for first in points {
        for second in points {
            if first == second { continue; }
            
            let diff = *second - *first;

            result.push(*second + diff);
            result.push(*first - diff);

        }
    }

    result
}

fn load_file(file: &str) -> Result<(GridData, i32, i32)> {
    let input = File::open(file)?;
    let mut grid: GridData = HashMap::new();

    let char_grid = BufReader::new(input)
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .collect::<Vec<_>>()
        }).collect::<Vec<_>>();


    char_grid.iter()
        .enumerate()
        .for_each(|(y, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, value)| **value != '.')
                .for_each(|(x, value)| {
                    grid.entry(*value)
                        .or_insert_with(Vec::new)
                        .push(Point{
                            x: x as i32,
                            y: y as i32,
                        });
                });
        });

    let x = char_grid[0].len().try_into().unwrap();
    let y = char_grid.len().try_into().unwrap();

    Ok((grid, x, y))
}
