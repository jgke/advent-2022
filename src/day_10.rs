use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Noop,
    Addx(i32),
}

impl Instruction {
    fn new(s: &str) -> Instruction {
        match s {
            "noop" => Instruction::Noop,
            _ => {
                let split = s.split(' ').collect::<Vec<_>>();
                assert_eq!("addx", split[0]);
                Instruction::Addx(split[1].parse().unwrap())
            }
        }
    }
}

type Input = Vec<Instruction>;

fn one_impl(input: &Input) -> usize {
    let mut out = 0;
    let mut register = 1;
    let mut cycle = 1;
    for instr in input {
        match instr {
            Instruction::Noop => {
                if (cycle + 20) % 40 == 0 {
                    out += register * cycle;
                }
                cycle += 1;
            }
            Instruction::Addx(i) => {
                if (cycle + 20) % 40 == 0 {
                    out += register * cycle;
                } else if (cycle + 21) % 40 == 0 {
                    out += register * (cycle + 1);
                }
                register += i;
                cycle += 2;
            }
        }
    }
    out as usize
}

fn do_print(register: i32, cycle: i32) {
    if cycle % 40 == 0 {
        println!();
    }
    if (register - (cycle % 40)).abs() > 1 {
        print!(".");
    } else {
        print!("#");
    }
}

fn two_impl(input: &Input) {
    let mut register = 1;
    let mut cycle = 0;
    for instr in input {
        match instr {
            Instruction::Noop => {
                do_print(register, cycle);
                cycle += 1;
            }
            Instruction::Addx(i) => {
                do_print(register, cycle);
                do_print(register, cycle + 1);
                register += i;
                cycle += 2;
            }
        }
    }
    println!();
}

fn parse(reader: Vec<String>) -> Input {
    reader
        .into_iter()
        .map(|s| Instruction::new(s.as_str()))
        .collect()
}

pub fn ten() -> Result<(), std::io::Error> {
    let file = File::open("input/10_input")?;
    let reader = BufReader::new(file);
    let input = parse(reader.lines().map(|t| t.unwrap()).collect());
    println!("Day 10 part 1: {}", one_impl(&input));
    println!("Day 10 part 2: {:?}", two_impl(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_10::{one_impl, parse, two_impl};

    #[test]
    fn it_works() {
        let input = "
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
"
        .trim()
        .lines()
        .collect::<Vec<_>>();
        let parsed = parse(input.iter().map(|s| s.to_string()).collect());
        assert_eq!(13140, one_impl(&parsed));
        assert_eq!((), two_impl(&parsed));
    }
}
