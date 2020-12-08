use advent_of_code::file_lines;
use anyhow::Result;
use regex::Regex;

fn main() -> Result<()> {
    let sol = part1()?;
    println!("Part one answer: {}", sol);

    let sol = part1b()?;
    println!("Part one answer: {}", sol);

    let sol = part1c()?;
    println!("Part one answer: {}", sol);

    // let sol = part2c()?;
    // println!("Part two answer: {}", sol);

    Ok(())
}

fn part1() -> Result<i32> {
    let lines = file_lines("./input/day08/input.txt")?;
    let lines: Vec<_> = lines.map(|l| l).collect();
    let mut check = vec![false; lines.len()];
    let re = Regex::new(r"(?P<op>[a-z ]{3}) (?P<sign>[+-])(?P<num>\d+)").unwrap();
    let mut counter = 0;

    let mut i = 0;
    loop {
        if i == lines.len() || check[i] == true {
            break;
        }
        check[i] = true;
        let captures = re.captures(&lines[i]).unwrap();

        let mut op = String::new();
        let mut sign = String::new();
        let mut num = String::new();

        captures.expand("$op", &mut op);
        captures.expand("$sign", &mut sign);
        captures.expand("$num", &mut num);

        if op == "acc" {
            if sign == "+" {
                counter += num.parse::<i32>().unwrap();
            } else {
                counter -= num.parse::<i32>().unwrap();
            }
            i += 1;
        } else if op == "jmp" {
            if sign == "+" {
                i += num.parse::<usize>().unwrap();
            } else {
                i -= num.parse::<usize>().unwrap();
            }
        } else if op == "nop" {
            i += 1;
        }
    }

    Ok(counter)
}

#[derive(Clone, Debug)]
enum Command {
    ACC(i32, i32, usize),
    JMP(i32, i32, usize),
    NOP(i32, i32, usize),
}

#[derive(Clone, Debug)]
enum Command2 {
    ACC(i32, bool),
    JMP(i32, bool),
    NOP(i32, bool),
}

fn part1b() -> Result<i32> {
    let lines = file_lines("./input/day08/input.txt")?;
    let mut commands = Vec::new();

    let re = Regex::new(r"(?P<op>[a-z ]{3}) (?P<sign>[+-])(?P<num>\d+)").unwrap();

    for line in lines {
        let captures = re.captures(&line).unwrap();

        let mut op = String::new();
        let mut sign = String::new();
        let mut num = String::new();

        captures.expand("$op", &mut op);
        captures.expand("$sign", &mut sign);
        captures.expand("$num", &mut num);

        let n;
        if sign == "+" {
            n = num.parse::<i32>().unwrap();
        } else {
            n = -num.parse::<i32>().unwrap();
        }

        if op == "acc" {
            commands.push(Some(Command::ACC(n, 0, 0)));
        } else if op == "jmp" {
            commands.push(Some(Command::JMP(n, 0, 0)));
        } else if op == "nop" {
            commands.push(Some(Command::NOP(n, 0, 0)));
        }
    }

    let mut execution_history: Vec<Option<Command>> = Vec::new();
    let mut counter = 0;
    let mut pc = 0;
    let mut check = vec![false; commands.len()];

    loop {
        if pc == commands.len() || check[pc] == true {
            break;
        }
        check[pc] = true;

        execution_history.push(commands[pc].clone());

        match commands[pc] {
            Some(Command::ACC(n, _, _)) => {
                commands[pc] = Some(Command::ACC(n, counter, pc));
                counter = counter + n;
                pc += 1;
            }
            Some(Command::JMP(n, _, _)) => {
                commands[pc] = Some(Command::JMP(n, counter, pc));
                pc = if n > 0 {
                    pc + n as usize
                } else {
                    pc - ((-n) as usize)
                }
            }
            Some(Command::NOP(n, _, _)) => {
                commands[pc] = Some(Command::NOP(n, counter, pc));
                pc += 1
            }
            None => unreachable!(),
        };
    }

    Ok(counter)
}

fn part1c() -> Result<i32> {
    let lines = file_lines("./input/day08/input.txt")?;
    let mut commands = Vec::new();

    let re = Regex::new(r"(?P<op>[a-z ]{3}) (?P<sign>[+-])(?P<num>\d+)").unwrap();

    for line in lines {
        let captures = re.captures(&line).unwrap();

        let mut op = String::new();
        let mut sign = String::new();
        let mut num = String::new();

        captures.expand("$op", &mut op);
        captures.expand("$sign", &mut sign);
        captures.expand("$num", &mut num);

        let n;
        if sign == "+" {
            n = num.parse::<i32>().unwrap();
        } else {
            n = -num.parse::<i32>().unwrap();
        }

        if op == "acc" {
            commands.push(Some(Command2::ACC(n, false)));
        } else if op == "jmp" {
            commands.push(Some(Command2::JMP(n, false)));
        } else if op == "nop" {
            commands.push(Some(Command2::NOP(n, false)));
        }
    }

    part1d(&mut commands, 0, 0)
}

fn part1d(commands: &mut Vec<Option<Command2>>, counter: i32, pc: usize) -> Result<i32> {
    match commands[pc] {
        Some(Command2::ACC(_, true)) => Ok(counter),
        Some(Command2::JMP(_, true)) => Ok(counter),
        Some(Command2::NOP(_, true)) => Ok(counter),
        Some(Command2::ACC(n, false)) => {
            commands[pc] = Some(Command2::ACC(n, true));
            part1d(commands, counter + n, pc + 1)
        }
        Some(Command2::JMP(n, false)) => {
            commands[pc] = Some(Command2::JMP(n, true));
            part1d(commands, counter, (pc as i32 + n) as usize)
        }
        Some(Command2::NOP(n, false)) => {
            commands[pc] = Some(Command2::NOP(n, true));
            part1d(commands, counter, pc + 1)
        }
        None => unreachable!(),
    }
}

