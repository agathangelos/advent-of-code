use advent_of_code::file_lines;
use anyhow::{Result};

fn main() -> Result<()> {
    let lines = file_lines("./input/day03/input.txt")?;
    let map: Vec<Vec<_>> = lines.map(|l| l.chars().collect()).collect();

    let sol1 = solve_part1(&map, 3, 1);
    println!("sol 1: {}", sol1);

    let sol2 = solve_part2(&map);
    println!("sol 1: {}", sol2);

    Ok(())
}

fn is_tree(map: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    map[y % map.len()][x % map[0].len()] == '#'
}

fn solve_part1(map: &Vec<Vec<char>>, right_inc: usize, down_inc: usize) -> u32 {
    let mut trees = 0;
    
    let mut x = 0;
    let mut y = 0;

    while y < map.len() {
        if is_tree(map, x, y) {
            trees += 1;
        }

        x += right_inc;
        y += down_inc;
    }

    trees
}

fn solve_part2(map: &Vec<Vec<char>>) -> u32 {
    let slopes = vec![(1,1), (3,1), (5, 1), (7, 1), (1, 2)];

    let mut res = 1;
    for slope in slopes {
        res *= solve_part1(map, slope.0, slope.1);
    }

    res
}