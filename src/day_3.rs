use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

type Input = Vec<String>;

fn one_impl(input: &Input) -> i32 {
    let mut sum = 0;
    for line in input {
        let (first, second) = line.split_at(line.len() / 2);
        let f = first.chars().collect::<HashSet<_>>();
        let s = second.chars().collect::<HashSet<_>>();
        sum += f
            .intersection(&s)
            .map(|c| {
                if c.is_ascii_uppercase() {
                    (*c as i32) - ('A' as i32) + 27
                } else {
                    (*c as i32) - ('a' as i32) + 1
                }
            })
            .sum::<i32>();
    }
    sum
}

fn two_impl(input: &Input) -> i32 {
    let mut sum = 0;
    for chunk in input.chunks(3) {
        sum += chunk
            .iter()
            .map(|s| s.chars().collect::<HashSet<_>>())
            .reduce(|a, b| a.intersection(&b).copied().collect())
            .unwrap()
            .iter()
            .map(|c| {
                if c.is_ascii_uppercase() {
                    (*c as i32) - ('A' as i32) + 27
                } else {
                    (*c as i32) - ('a' as i32) + 1
                }
            })
            .sum::<i32>();
    }
    sum
}

fn parse(reader: Vec<String>) -> Input {
    reader
}

pub fn three() -> Result<(), std::io::Error> {
    let file = File::open("input/3_input")?;
    let reader = BufReader::new(file);
    let input = parse(reader.lines().map(|t| t.unwrap()).collect());
    println!("Day 3 part 1: {}", one_impl(&input));
    println!("Day 3 part 2: {}", two_impl(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_3::{one_impl, parse, two_impl};

    #[test]
    fn it_works() {
        let input = [
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ];
        let parsed = parse(input.iter().map(|s| s.to_string()).collect());
        assert_eq!(157, one_impl(&parsed));
        assert_eq!(70, two_impl(&parsed));
    }
}
