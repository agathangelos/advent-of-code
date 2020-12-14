use advent_of_code::file_lines;
use anyhow::Result;
use regex::Regex;

fn main() -> Result<()> {
    let input = file_lines("./input/day02/input.txt")?;

    let lines: Vec<_> = input.map(|l| l).collect();
    let re = Regex::new(r"(?P<min>\d+)-(?P<max>\d+)\s+(?P<target>\w): (?P<password>\w+)").unwrap();

    let sol1 = solve_part1(&lines, &re)?;
    println!("Part one answer: {}", sol1);

    let sol2 = solve_part2(&lines, &re)?;
    println!("Part two answer: {}", sol2);

    Ok(())
}

fn solve_part1(lines: &Vec<String>, re: &Regex) -> Result<u32> {
    let mut correct_passwords = 0;

    for l in lines {
        let captures = re.captures(l.as_str()).unwrap();

        let mut min = String::new();
        let mut max = String::new();
        let mut target = String::new();
        let mut password = String::new();

        captures.expand("$min", &mut min);
        captures.expand("$max", &mut max);
        captures.expand("$target", &mut target);
        captures.expand("$password", &mut password);

        let target = target.chars().next().unwrap();

        let count = password
            .chars()
            .into_iter()
            .filter(|c| *c == target)
            .count();
        if count >= min.parse().unwrap() && count <= max.parse().unwrap() {
            correct_passwords += 1;
        }
    }

    Ok(correct_passwords)
}

fn solve_part2(lines: &Vec<String>, re: &Regex) -> Result<u32> {
    let mut correct_passwords = 0;

    for l in lines {
        let captures = re.captures(l.as_str()).unwrap();

        let mut min = String::new();
        let mut max = String::new();
        let mut target = String::new();
        let mut password = String::new();

        captures.expand("$min", &mut min);
        captures.expand("$max", &mut max);
        captures.expand("$target", &mut target);
        captures.expand("$password", &mut password);

        let target = target.chars().next().unwrap();
        let a = password
            .chars()
            .nth(min.parse::<usize>().unwrap() - 1 as usize)
            .unwrap();
        let b = password
            .chars()
            .nth(max.parse::<usize>().unwrap() - 1 as usize)
            .unwrap();

        if (a == target) ^ (b == target) {
            correct_passwords += 1;
        }
    }

    Ok(correct_passwords)
}
