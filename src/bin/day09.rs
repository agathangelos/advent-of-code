use advent_of_code::file_lines;
use anyhow::{bail, Result};

fn main() -> Result<()> {
    let lines = file_lines("./input/day09/input.txt")?;
    let numbers: Vec<_> = lines.map(|n| n.parse::<i64>().unwrap()).collect();

    let sol = part_1(&numbers[0..numbers.len()])?;
    println!("Part one answer: {}", sol);

    let sol = part_2(&numbers[0..numbers.len()], sol)?;
    println!("Part two answer: {}", sol);

    Ok(())
}

fn part_1(numbers: &[i64]) -> Result<i64> {
    for n1 in 25..numbers.len() {
        let subset = &numbers[1..n1];
        let mut okk = false;
        for n2 in 1..subset.len() {
            let n = subset.iter().find(|&&x| x == (numbers[n1] - subset[n2]));
            if let Some(_) = n {
                okk = true;
            }
        }
        if okk == false {
            return Ok(numbers[n1]);
        }
    }

    bail!("unreachable");
}

fn part_2(numbers: &[i64], target: i64) -> Result<i64> {
    for n1 in 25..numbers.len() {
        let subset = &numbers[1..n1];
        let sum = subset.iter().fold(0, |acc, x| acc + x);

        if sum == target {
            return Ok(subset[0] + subset[subset.len() - 1]);
        } else if sum > target {
            for i in 0..n1 {
                let subset = &numbers[i..n1];
                let sum = subset.iter().fold(0, |acc, x| acc + x);
                if sum == target {
                    return Ok(subset.iter().min().unwrap() + subset.iter().max().unwrap());
                }
            }
        }
    }

    bail!("unreachable");
}
