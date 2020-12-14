use advent_of_code::file_lines;
use anyhow::{bail, Result};

fn main() -> Result<()> {
    let input = file_lines("./input/day01/input.txt")?;
    let numbers: Vec<u32> = input.map(|n| n.parse::<u32>().unwrap()).collect();
    // let mut v = &numbers;

    let sol2 = solve_part2(&numbers, 2020)?;
    println!("Part two answer: {}", sol2);

    let sol1 = solve_part1(numbers, 2020)?;
    println!("Part one answer: {}", sol1);

    Ok(())
}

fn solve_part1(mut numbers: Vec<u32>, target: u32) -> Result<u32> {
    &numbers.sort();
    let len = numbers.len();
    for i in 0..len {
        let ii = (&numbers)[i];
        if let Some(i) = (&numbers).into_iter().find(|&n| *n == (target - ii)) {
            return Ok(ii * i);
        }
    }

    bail!("unreachable");
}

fn solve_part2(numbers: &Vec<u32>, target: u32) -> Result<u32> {
    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            for k in 0..numbers.len() {
                if numbers[i] + numbers[j] + numbers[k] == target {
                    return Ok(numbers[i] * numbers[j] * numbers[k]);
                }
            }
        }
    }

    bail!("unreachable");
}
