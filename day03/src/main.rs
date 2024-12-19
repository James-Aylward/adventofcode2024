use std::fs;
use regex::Regex;

fn main() -> anyhow::Result<()> {


    let contents = fs::read_to_string("day03/input.txt")
        .unwrap();

    let mut re = Regex::new(r"mul\((\d+),(\d+)\)")
        .unwrap();

    let part_a: u32 = re.captures_iter(&contents)
        .map(|cap| {
            let a: u32 = cap[1].parse().ok().unwrap();
            let b: u32 = cap[2].parse().ok().unwrap();
            a * b
        }).sum();

    println!("{:?}", part_a);

    re = Regex::new(r"mul\((\d+),(\d+)\)|don't()|do()")
        .unwrap();

    let mut flag = true;

    let part_b: u32 = re.captures_iter(&contents)
        .map(|cap| {

            let mut r = 0;
            match cap[0].parse::<String>().ok() {
                Some(v) if v == "do" => flag = true,
                Some(v) if v == "don't" => flag = false,
                _  if flag => {
                    r = cap[1].parse::<u32>().unwrap()
                        * cap[2].parse::<u32>().unwrap()
                },
                _ => (),
            }
            r

        }).sum();

    println!("{:?}", part_b);

    Ok(())
}
