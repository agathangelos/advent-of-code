use advent_of_code::file_lines;
use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let (unique, everyone) = solution()?;
    println!("Part one answer: {}", unique);
    println!("Part two answer: {}", everyone);

    Ok(())
}

fn solution() -> Result<(u32, u32)> {
    let lines = file_lines("./input/day06/input.txt")?;
    let mut everyone = 0;
    let mut unique: u32 = 0;

    for (key, group) in &lines.into_iter().group_by(|x| x != "") {
        if key {
            let mut v = vec![0u32; 26];
            let mut size = 0;
            for l in group.into_iter() {
                for c in l.chars() {
                    v[c as usize - 'a' as usize] = v[c as usize - 'a' as usize] + 1;
                }
                size = size + 1;
            }

            let t: u32 = v.iter().map(|x| if *x > 0 { 1 } else { 0 }).sum();
            unique = unique + t;

            everyone = everyone + v.iter().filter(|&x| *x == size).count();
        }
    }

    Ok((unique, everyone as u32))
}
