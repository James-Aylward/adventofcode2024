use std::{fs::File, io::{BufRead, BufReader}};

fn find_word(
    grid: &Vec<Vec<char>>,
    dir: &(i32, i32),
    word: &str,
    r: usize,
    c: usize
) -> bool {
    word
        .chars()
        .enumerate()
        .map(|(i, v)| {

            let x = (i as i32)
                .checked_mul(dir.0)
                .and_then(|x| x.checked_add(c as i32))
                .and_then(|x| if x >= 0 { Some(x) } else { None });
            let y = (i as i32)
                .checked_mul(dir.1)
                .and_then(|x| x.checked_add(r as i32))
                .and_then(|x| if x >= 0 { Some(x) } else { None });

            if let (Some(x), Some(y)) = (x, y) {
                // Index into row and column here. Return item == v
                let item = grid
                    .get(y as usize)
                    .and_then(|row: &Vec<char>| row.get(x as usize).copied());
                return item == Some(v)
            } else {
                return false
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

    let word = "XMAS";
    let directions = [
        (-1, 0), // back
        (1, 0), // forwards
        (0, 1), // up
        (0, -1), // down
        (1, 1), // top right
        (-1, 1), // top left
        (1, -1), // bottom right
        (-1, -1), // bottom left
    ];

    let mut count: u32 = 0;

    for r in 0..grid.len() {
        for c in 0..grid[r].len() {

            // Skip if not an X
            if grid[r][c] != word.chars().nth(0).unwrap() { continue; }

            count += directions
                .iter()
                .map(|dir| {
                    find_word(&grid, &dir, &word, r, c)
                }).fold(0, |acc, x| acc + x as u32);
        }
    }

    println!("Count is {}", count);

    Ok(())
}

