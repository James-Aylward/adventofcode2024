use anyhow::Result;
use itertools::Itertools;
use std::{thread, fs::File, io::{BufRead, BufReader}};

#[derive(Clone,Hash,Eq,PartialEq)]
enum Direction {
    North,
    East,
    South,
    West
}

impl Direction {
    fn rotate(self: &mut Direction) {
        *self = match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn get_offset(&self) -> (i32, i32) {
        match self {
            Direction::North => (0, -1),
            Direction::East => (1, 0), 
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        }
    }
}

#[derive(Hash,Eq,PartialEq)]
struct Snapshot {
    pos: (usize, usize),
    dir: Direction,
}

fn main() -> Result<()> {
    let input = File::open("day06/input.txt")?;
    //let input = File::open("day06/test.txt")?;
    let grid = BufReader::new(input)
        .lines()
        .map(|l| l.unwrap().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let path = patrol(&grid).1;
    let part_a = path.iter()
        .unique_by(|x| x.pos)
        .count();
    println!("Part a is {}", part_a);

    let mut handles = Vec::new();
    path.iter()
        .unique_by(|x| x.pos)
        .filter(|x| x.pos != get_start(&grid).unwrap())
        .for_each(|s| {
            let (x, y) = s.pos;
            let mut modified = grid.to_vec();
            modified[y][x] = '#';
            let handle = thread::spawn(move || {

                patrol(&modified).0 as u32
            });
            handles.push(handle);
        });

    let part_b: u32 = handles.into_iter()
        .map(|h| h.join().unwrap())
        .sum();

    println!("Part b is {}", part_b);
    //todo!();

    Ok(())
}

fn print_grid(grid: &Vec<Vec<char>>) {
    grid.iter()
        .for_each(|line| {
            let string: String = line.into_iter()
                .collect();
            println!("{}", string);
        });
}

fn get_ahead(
    direction: &Direction, 
    grid: &Vec<Vec<char>>, 
    mut pos: (usize, usize)
) -> Option<char> {
    let (off_x, off_y) = direction.get_offset();
    pos.0 = (pos.0 as i32 + off_x) as usize;
    pos.1 = (pos.1 as i32 + off_y) as usize;
    grid.get(pos.1 as usize)
        .and_then(|row| row.get(pos.0 as usize).copied())
}

// Returns whether there is a loop, and the visited path
fn patrol(grid: &Vec<Vec<char>>) -> (bool, Vec<Snapshot>) {
    let mut explored: Vec<Snapshot> = Vec::new();
    let mut dir = Direction::North;
    let mut pos = get_start(grid).unwrap();

    loop {
        let current = Snapshot{pos, dir: dir.clone()};
        if explored.contains(&current) {
            return (true, explored)
        }
        explored.push(current);

        if let Some(c) = get_ahead(&dir, &grid, pos) {
            if c == '#' { dir.rotate() }
            let off = dir.get_offset();
            pos.0 = (pos.0 as i32 + off.0) as usize;
            pos.1 = (pos.1 as i32 + off.1) as usize;

        } else  {
            return (false, explored)
        }
    }

}

fn get_start(grid: &Vec<Vec<char>>) -> Result<(usize, usize)> {
    for (row_index, row) in grid.iter().enumerate() {
        for (column_index, value) in row.iter().enumerate() {
            if *value == '^' {
                return Ok((column_index, row_index));
            }
        }
    }
    Err(anyhow::anyhow!("Couldn't find start"))
}

