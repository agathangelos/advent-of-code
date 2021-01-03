use advent_of_code::file_lines;
use anyhow::Result;

fn main() -> Result<()> {
    let lines = file_lines("./input/day13/input.txt")?;

    let mut lines_iter = lines.into_iter();
    let timestamp = lines_iter.next().unwrap().parse::<u32>().unwrap();
    let buses = lines_iter.next().unwrap();

    let res = part1(timestamp, &buses)?;
    println!("-->{:?}", res);

    // let res = part2(&instructions);
    // println!("-->{:?}", res);

    Ok(())
}

fn part1(timestamp: u32, buses: &str) -> Result<u32> {
    let buses: Vec<_> = buses
        .split(",")
        .into_iter()
        .filter_map(|x| x.parse::<u32>().ok())
        .collect();

    let mut mod_res = Vec::new();
    for i in &buses {
        mod_res.push(i - timestamp % i);
    }

    let (minutes, idx) = mod_res
        .iter()
        .enumerate()
        .map(|(x, y)| (y, x))
        .min()
        .unwrap();

    Ok(buses[idx] * minutes)
}
