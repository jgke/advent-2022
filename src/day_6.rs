use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

type Input = Vec<char>;

fn one_impl(input: &Input) -> usize {
    for i in 0..input.len() - 3 {
        let buf = input[i..i + 4].iter().collect::<HashSet<_>>();
        if buf.len() == 4 {
            return i + 4;
        }
    }
    unimplemented!()
}

fn two_impl(input: &Input) -> usize {
    for i in 0..input.len() - 14 {
        let buf = input[i..i + 14].iter().collect::<HashSet<_>>();
        if buf.len() == 14 {
            return i + 14;
        }
    }
    unimplemented!()
}

fn parse(reader: Vec<String>) -> Input {
    reader[0].chars().collect()
}

pub fn six() -> Result<(), std::io::Error> {
    let file = File::open("6_input")?;
    let reader = BufReader::new(file);
    let input = parse(reader.lines().map(|t| t.unwrap()).collect());
    println!("Day 6 part 1: {}", one_impl(&input));
    println!("Day 6 part 2: {}", two_impl(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_6::{one_impl, parse, two_impl};

    #[test]
    fn it_works() {
        let input = ["mjqjpqmgbljsphdztnvjfqwrcgsmlb"];
        let parsed = parse(input.iter().map(|s| s.to_string()).collect());
        assert_eq!(7, one_impl(&parsed));
        assert_eq!(19, two_impl(&parsed));
    }
}
