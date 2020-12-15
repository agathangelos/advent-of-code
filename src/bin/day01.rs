use advent_of_code::file_lines;
use anyhow::{bail, Result};

fn main() -> Result<()> {
    let input = file_lines("./input/day01/input.txt")?;
    let mut numbers: Vec<u32> = input.map(|n| n.parse::<u32>().unwrap()).collect();
    numbers.sort();
    let numbers = numbers;

    let sol1 = solve_part1(&numbers, 2020)?;
    println!("Part one answer: {}", sol1);

    let sol2 = solve_part2(&numbers, 2020)?;
    println!("Part two answer: {}", sol2);

    Ok(())
}

fn solve_part1(numbers: &Vec<u32>, target: u32) -> Result<u32> {
    for i in 0..numbers.len() {
        numbers.iter().find(|&n| *n == (target - numbers[i]));
        if let Some(n) = &numbers.iter().find(|&n| *n == (target - numbers[i])) {
            return Ok(numbers[i] * *n);
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
