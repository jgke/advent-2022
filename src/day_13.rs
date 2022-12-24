use std::cmp::Ordering;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::iter::Peekable;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Token {
    Num(i32),
    Op,
    Cl,
}

fn tokenize(s: &str) -> Vec<Token> {
    let mut out = Vec::new();
    let mut iter = s.chars().peekable();
    while let Some(c) = iter.next() {
        if c == ',' {
            continue;
        } else if c == '[' {
            out.push(Token::Op);
        } else if c == ']' {
            out.push(Token::Cl);
        } else {
            let mut num = c.to_string();
            while iter.peek().unwrap().is_ascii_digit() {
                num.push(iter.next().unwrap());
            }
            if iter.peek() == Some(&',') {
                iter.next();
            }
            out.push(Token::Num(num.parse().unwrap()));
        }
    }
    out
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Data {
    Num(i32),
    List(Vec<Data>),
}

impl Ord for Data {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Data::Num(n1), Data::Num(n2)) => n1.cmp(n2),
            (Data::List(l1), Data::List(l2)) => l1.cmp(l2),
            (Data::Num(n1), list) => Data::List(vec![Data::Num(*n1)]).cmp(list),
            (list, Data::Num(n2)) => list.cmp(&Data::List(vec![Data::Num(*n2)])),
        }
    }
}

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn recur<I>(iter: &mut Peekable<I>) -> Data
where
    I: Iterator<Item = Token>,
{
    match iter.next().unwrap() {
        Token::Num(n) => Data::Num(n),
        Token::Op => {
            let mut vec = Vec::new();
            while iter.peek().unwrap() != &Token::Cl {
                vec.push(recur(iter));
            }
            assert_eq!(Token::Cl, iter.next().unwrap());
            Data::List(vec)
        }
        Token::Cl => unreachable!(),
    }
}

type Input = Vec<(Data, Data)>;

fn one_impl(input: &Input) -> usize {
    let mut correct = 0;
    for (i, (first, second)) in input.iter().enumerate() {
        if first < second {
            correct += i + 1;
        }
    }
    correct
}

fn two_impl(input: &Input) -> usize {
    let marker1 = Data::List(vec![Data::List(vec![Data::Num(2)])]);
    let marker2 = Data::List(vec![Data::List(vec![Data::Num(6)])]);
    let mut list = input
        .iter()
        .flat_map(|(f, s)| [f.clone(), s.clone()])
        .collect::<Vec<Data>>();
    list.push(marker1.clone());
    list.push(marker2.clone());
    list.sort();
    (list.iter().position(|m| m == &marker1).unwrap() + 1)
        * (list.iter().position(|m| m == &marker2).unwrap() + 1)
}

fn parse(reader: Vec<String>) -> Input {
    reader
        .chunks(3)
        .map(|chunk| {
            let first = recur(&mut tokenize(&chunk[0]).into_iter().peekable());
            let second = recur(&mut tokenize(&chunk[1]).into_iter().peekable());
            (first, second)
        })
        .collect()
}

pub fn thirteen() -> Result<(), std::io::Error> {
    let file = File::open("13_input")?;
    let reader = BufReader::new(file);
    let input = parse(reader.lines().map(|t| t.unwrap()).collect());
    println!("Day 13 part 1: {}", one_impl(&input));
    println!("Day 13 part 2: {}", two_impl(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_13::{one_impl, parse, two_impl};

    #[test]
    fn it_works() {
        let input = "
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
"
        .trim()
        .lines()
        .collect::<Vec<_>>();
        let parsed = parse(input.iter().map(|s| s.to_string()).collect());
        assert_eq!(13, one_impl(&parsed));
        assert_eq!(140, two_impl(&parsed));
    }
}
