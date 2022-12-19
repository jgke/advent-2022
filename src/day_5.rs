use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

type Input = (Vec<Vec<char>>, Vec<(usize, usize, usize)>);

fn one_impl(input: &Input) -> String {
    let (mut crates, operations) = input.clone();
    for (count, from, to) in operations {
        for _ in 0..count {
            let p = crates[from].pop().unwrap();
            crates[to].push(p);
        }
    }
    crates.iter().map(|c| c.last().unwrap()).collect()
}

fn two_impl(input: &Input) -> String {
    let (mut crates, operations) = input.clone();
    for (count, from, to) in operations {
        let mut popped = Vec::new();
        for _ in 0..count {
            popped.push(crates[from].pop().unwrap());
        }
        popped.reverse();
        crates[to].extend(&mut popped.into_iter());
    }
    crates.iter().map(|c| c.last().unwrap()).collect()
}

fn parse(reader: Vec<String>) -> Input {
    let mut crates = Vec::new();
    let mut operations = Vec::new();
    let mut iter = reader.iter();
    while let Some(s) = iter.next() {
        if s == "" {
            break;
        }
        let mut vec = s.chars().collect::<Vec<_>>();
        vec.reverse();
        crates.push(vec);
    }
    while let Some(s) = iter.next() {
        let parts = s.split(" ").collect::<Vec<_>>();
        operations.push((
            parts[1].parse().unwrap(),
            parts[3].parse::<usize>().unwrap() - 1,
            parts[5].parse::<usize>().unwrap() - 1,
        ));
    }
    (crates, operations)
}

pub fn five() -> Result<(), std::io::Error> {
    let file = File::open("5_input")?;
    let reader = BufReader::new(file);
    let input = parse(reader.lines().map(|t| t.unwrap()).collect());
    println!("Day 4 part 1: {}", one_impl(&input));
    println!("Day 4 part 2: {}", two_impl(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_5::{one_impl, parse, two_impl};

    #[test]
    fn it_works() {
        let input = [
            "NZ",
            "DCM",
            "P",
            "",
            "move 1 from 2 to 1",
            "move 3 from 1 to 3",
            "move 2 from 2 to 1",
            "move 1 from 1 to 2",
        ];
        let parsed = parse(input.iter().map(|s| s.to_string()).collect());
        assert_eq!("CMZ", one_impl(&parsed));
        assert_eq!("MCD", two_impl(&parsed));
    }
}
