use advent_of_code::file_lines;
use anyhow::{Result, bail};

fn main() -> Result<()> {
    let v = solve_part1()?;
    println!("Part one answer: {}", v[v.len() - 1]);

    let idx = solve_part2(v)?;
    println!("Idx: {}", idx);
    
    Ok(())
}

fn solve_part1() -> Result<Vec<u32>> {
    let lines = file_lines("./input/day05/input.txt")?;
    let mut v: Vec<u32> = Vec::new();

    for l in lines {
        let n = binary_search((l[0..7]).to_string(), 128, 0, 'B', 'F');

        let x = binary_search((l[7..10]).to_string(), 8, 0, 'R', 'L');

        let idx = n.unwrap() * 8 + x.unwrap();

        v.push(idx);
    }

    v.sort();
    Ok(v)
}

fn binary_search(s: String, range: u32, idx: u32, upper: char, lower: char) -> Result<u32> {
    if s.len() > 0 {
        let i;
        let c = s.chars().nth(0).unwrap();
        if c == upper {
            i = idx + range / 2;
        } else if c == lower {
            i = idx;
        } else {
            bail!("unreachable {}", s);
        }
        let range = range / 2;
        binary_search((s[1..]).to_string(), range, i, upper, lower)
    } else {
        Ok(idx)
    }
}
 
// TODO solve with binary search
fn solve_part2(v: Vec<u32>) -> Result<u32> {
    for i in 1..v.len() {
        if v[i-1]+1 != v[i] {
            return Ok(v[i] - 1);
        }
    }

    bail!("unreachable");
}

