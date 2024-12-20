use std::{fs::File, io::{BufRead, BufReader}};

fn find_cross(
    grid: &Vec<Vec<char>>,
    row: usize,
    column: usize,
    directions: &[(i32, i32)]
) -> bool {

    if grid[row][column] != 'A' {
        return false;
    }

    let letters = directions.iter().map(|dir| {
        if let Some((x, y)) = get_position(row, column, &dir, 1) {
            grid.get(y as usize)
                .and_then(|row: &Vec<char>| row.get(x as usize).copied())
        } else { None }
    }).collect::<Vec<_>>();

    let a: String = letters.iter()
        .take(2)
        .map(|x| x.unwrap_or('_'))
        .collect();

    let b: String = letters.iter()
        .rev()
        .take(2)
        .map(|x| x.unwrap_or('_'))
        .collect();

    let combos = ["MS", "SM"];
    combos.contains(&a.as_str()) && combos.contains(&b.as_str())
}

fn get_position(
    row: usize,
    column: usize,
    direction: &(i32, i32),
    distance: usize
) -> Option<(usize, usize)> {
    let x: Option<usize> = (distance as i32)
        .checked_mul(direction.0)
        .and_then(|x| x.checked_add(column as i32))
        .filter(|&x| x >= 0)
        .and_then(|x| x.try_into().ok());

    let y: Option<usize> = (distance as i32)
        .checked_mul(direction.1)
        .and_then(|x| x.checked_add(row as i32))
        .filter(|&x| x >= 0)
        .and_then(|x| x.try_into().ok());

    // Returns Option<(usize,usize) where is one is None, return is None
    x.zip(y)
}

fn find_word(
    grid: &Vec<Vec<char>>,
    direction: &(i32, i32),
    row: usize,
    column: usize
) -> bool {
    "XMAS"
        .chars()
        .enumerate()
        .map(|(i, v)| {
            if let Some((x, y)) = get_position(row, column, direction, i) {
                // Index into row and column here. Return item == v
                grid.get(y as usize)
                    .and_then(|row: &Vec<char>| row.get(x as usize).copied())
                    == Some(v)
            } else {
                false
            }
        }).all(|x| x)
}

fn main() -> anyhow::Result<()> {

    let input = File::open("day04/input.txt")?;
    //let input = File::open("day04/test.txt")?;
    let grid = BufReader::new(input)
        .lines()
        .map(|l| l.unwrap().chars().collect::<Vec<_>>() )
        .collect::<Vec<_>>();

    let orthogonals = [
        (-1, 0), // back
        (1, 0), // forwards
        (0, 1), // up
        (0, -1), // down
    ];
    let diagonals = [
        (-1, 1), // top left
        (1, -1), // bottom right
        (1, 1), // top right
        (-1, -1), // bottom left
    ];

    let mut count_a: u32 = 0;
    let mut count_b: u32 = 0;

    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            count_a += [orthogonals, diagonals].concat()
                .iter()
                .map(|dir| find_word(&grid, &dir, r, c))
                .fold(0, |acc, x| acc + x as u32);
            count_b += find_cross(&grid, r, c, &diagonals) as u32;
        }
    }

    println!("Count is {}", count_a);
    println!("Count is {}", count_b);

    Ok(())
}
