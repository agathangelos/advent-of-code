use advent_of_code::file_lines;
use anyhow::Result;
use regex::Regex;

fn main() -> Result<()> {
    let sol = part_1a()?;
    println!("Part one answer: {}", sol);

    let sol = part_1b()?;
    println!("Part one answer: {}", sol);

    let sol = part_1c()?;
    println!("Part one answer: {}", sol);

    let sol = part2c()?;
    println!("Part two answer: {}", sol);

    Ok(())
}

fn part_1a() -> Result<i32> {
    let lines = file_lines("./input/day08/input.txt")?;
    let lines: Vec<_> = lines.map(|l| l).collect();
    let mut check = vec![false; lines.len()];
    let re = Regex::new(r"(?P<op>[a-z ]{3}) (?P<num>[+-]\d+)").unwrap();
    let mut counter = 0;

    let mut i = 0;
    loop {
        if i == lines.len() || check[i] == true {
            break;
        }
        check[i] = true;
        let captures = re.captures(&lines[i]).unwrap();

        let mut op = String::new();
        let mut num = String::new();

        captures.expand("$op", &mut op);
        captures.expand("$num", &mut num);

        if op == "acc" {
            counter += num.parse::<i32>().unwrap();
            i += 1;
        } else if op == "jmp" {
            i = (i as i32 + num.parse::<i32>().unwrap()) as usize;
        } else if op == "nop" {
            i += 1;
        }
    }

    Ok(counter)
}

#[derive(Clone, Debug)]
enum Command {
    ACC(i32),
    JMP(i32),
    NOP(i32),
}

#[derive(Clone, Debug)]
enum Command2 {
    ACC(i32, bool),
    JMP(i32, bool),
    NOP(i32, bool),
}

fn part_1b() -> Result<i32> {
    let lines = file_lines("./input/day08/input.txt")?;
    let mut commands = Vec::new();

    let re = Regex::new(r"(?P<op>[a-z ]{3}) (?P<num>[+-]\d+)").unwrap();

    for line in lines {
        let captures = re.captures(&line).unwrap();

        let mut op = String::new();
        let mut num = String::new();

        captures.expand("$op", &mut op);
        captures.expand("$num", &mut num);

        let n = num.parse::<i32>().unwrap();

        if op == "acc" {
            commands.push(Some(Command::ACC(n)));
        } else if op == "jmp" {
            commands.push(Some(Command::JMP(n)));
        } else if op == "nop" {
            commands.push(Some(Command::NOP(n)));
        }
    }

    let mut execution_history = Vec::new();
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
            Some(Command::ACC(n)) => {
                commands[pc] = Some(Command::ACC(n));
                counter = counter + n;
                pc += 1;
            }
            Some(Command::JMP(n)) => {
                commands[pc] = Some(Command::JMP(n));
                pc = (pc as i32 + n) as usize;
            }
            Some(Command::NOP(n)) => {
                commands[pc] = Some(Command::NOP(n));
                pc += 1
            }
            None => unreachable!(),
        };
    }

    Ok(counter)
}

fn part_1c() -> Result<i32> {
    let lines = file_lines("./input/day08/input.txt")?;
    let mut commands = Vec::new();

    let re = Regex::new(r"(?P<op>[a-z ]{3}) (?P<num>[+-]\d+)").unwrap();

    for line in lines {
        let captures = re.captures(&line).unwrap();

        let mut op = String::new();
        let mut num = String::new();

        captures.expand("$op", &mut op);
        captures.expand("$num", &mut num);

        let n = num.parse::<i32>().unwrap();

        if op == "acc" {
            commands.push(Some(Command2::ACC(n, false)));
        } else if op == "jmp" {
            commands.push(Some(Command2::JMP(n, false)));
        } else if op == "nop" {
            commands.push(Some(Command2::NOP(n, false)));
        }
    }

    Ok(run(&mut commands, 0, 0).unwrap())
}

fn run(commands: &mut Vec<Option<Command2>>, counter: i32, pc: usize) -> Option<i32> {
    if pc == commands.len() {
        return Some(counter);
    }

    match commands[pc] {
        Some(Command2::ACC(_, true)) => Some(counter),
        Some(Command2::JMP(_, true)) => Some(counter),
        Some(Command2::NOP(_, true)) => Some(counter),
        Some(Command2::ACC(n, false)) => {
            commands[pc] = Some(Command2::ACC(n, true));
            run(commands, counter + n, pc + 1)
        }
        Some(Command2::JMP(n, false)) => {
            commands[pc] = Some(Command2::JMP(n, true));
            run(commands, counter, (pc as i32 + n) as usize)
        }
        Some(Command2::NOP(n, false)) => {
            commands[pc] = Some(Command2::NOP(n, true));
            run(commands, counter, pc + 1)
        }
        None => unreachable!(),
    }
}

fn part2c() -> Result<i32> {
    let lines = file_lines("./input/day08/input.txt")?;
    let mut commands = Vec::new();

    let re = Regex::new(r"(?P<op>[a-z ]{3}) (?P<num>[+-]\d+)").unwrap();

    for line in lines {
        let captures = re.captures(&line).unwrap();

        let mut op = String::new();
        let mut num = String::new();

        captures.expand("$op", &mut op);
        captures.expand("$num", &mut num);

        let n = num.parse::<i32>().unwrap();

        if op == "acc" {
            commands.push(Some(Command2::ACC(n, false)));
        } else if op == "jmp" {
            commands.push(Some(Command2::JMP(n, false)));
        } else if op == "nop" {
            commands.push(Some(Command2::NOP(n, false)));
        }
    }

    let mut i = 0;
    let mut commands_run = commands.clone();
    loop {
        if i == commands.len() {
            break;
        }

        match run2(&mut commands_run, 0, 0) {
            Some(x) => {
                return Ok(x);
            }
            None => {
                commands_run = commands.clone();
                loop {
                    i += 1;
                    match commands[i] {
                        Some(Command2::JMP(n, v)) => {
                            commands_run[i] = Some(Command2::NOP(n, v));
                            break;
                        }
                        Some(Command2::NOP(n, v)) => {
                            commands_run[i] = Some(Command2::JMP(n, v));
                            break;
                        }
                        _ => {}
                    };
                }
            }
        };
    }

    Ok(0)
}

fn run2(commands: &mut Vec<Option<Command2>>, counter: i32, pc: usize) -> Option<i32> {
    if pc == commands.len() {
        return Some(counter);
    }

    match commands[pc] {
        Some(Command2::ACC(_, true)) => None,
        Some(Command2::JMP(_, true)) => None,
        Some(Command2::NOP(_, true)) => None,
        Some(Command2::ACC(n, false)) => {
            commands[pc] = Some(Command2::ACC(n, true));
            run2(commands, counter + n, pc + 1)
        }
        Some(Command2::JMP(n, false)) => {
            commands[pc] = Some(Command2::JMP(n, true));
            run2(commands, counter, (pc as i32 + n) as usize)
        }
        Some(Command2::NOP(n, false)) => {
            commands[pc] = Some(Command2::NOP(n, true));
            run2(commands, counter, pc + 1)
        }
        None => unreachable!(),
    }
}
