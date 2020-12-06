use advent_of_code::file_lines;
use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let count = solve_part1()?;
    println!("Part one answer: {}", count);
    let count = solve_part2()?;
    println!("Part two answer: {}", count);

    Ok(())
}

fn solve_part1() -> Result<u32> {
    let lines = file_lines("./input/day06/input.txt")?;

    let mut count = 0;
    let mut v = vec![0; 26];

    for l in lines.into_iter() {
        if l.is_empty() {
            // let s: u32 = v.iter().map(|x| if *x > 0 { 1 } else { 0 }).sum();
            let s: u32 = v.iter().sum();
            count = count + s;

            v = vec![0; 26];
        } else {
            for c in l.chars() {
                v[c as usize - 'a' as usize] = 1;
            }
        }
    }

    let s: u32 = v.iter().sum();
    count = count + s;

    Ok(count)
}

fn solve_part2() -> Result<u32> {
    let lines = file_lines("./input/day06/input.txt")?;
    let mut count = 0;

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

            count = count + v.iter().filter(|&x| *x == size).count();
        }
    }

    Ok(count as u32)
}
