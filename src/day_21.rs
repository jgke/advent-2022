use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

type Input = HashMap<String, Monkey>;
#[derive(Clone, Debug)]
enum Monkey {
    Const(i64),
    Human,
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum BoxMonkey {
    Const(i64),
    Human,
    Add(Box<BoxMonkey>, Box<BoxMonkey>),
    Sub(Box<BoxMonkey>, Box<BoxMonkey>),
    Mul(Box<BoxMonkey>, Box<BoxMonkey>),
    Div(Box<BoxMonkey>, Box<BoxMonkey>),
}

fn eval(monkey: &str, input: &HashMap<String, Monkey>, cache: &mut HashMap<String, i64>) -> i64 {
    if let Some(n) = cache.get(monkey) {
        return *n;
    }
    let n = match &input[monkey] {
        Monkey::Const(n) => *n,
        Monkey::Human => panic!(),
        Monkey::Add(left, right) => eval(left, input, cache) + eval(right, input, cache),
        Monkey::Sub(left, right) => eval(left, input, cache) - eval(right, input, cache),
        Monkey::Mul(left, right) => eval(left, input, cache) * eval(right, input, cache),
        Monkey::Div(left, right) => eval(left, input, cache) / eval(right, input, cache),
    };
    cache.insert(monkey.to_string(), n);
    n
}

fn one_impl(input: &Input) -> i64 {
    eval("root", input, &mut HashMap::new())
}

fn reduce(monkey: &BoxMonkey) -> BoxMonkey {
    match monkey {
        BoxMonkey::Const(n) => BoxMonkey::Const(*n),
        BoxMonkey::Human => BoxMonkey::Human,
        BoxMonkey::Add(left, right) => match (reduce(left), reduce(right)) {
            (BoxMonkey::Const(l), BoxMonkey::Const(r)) => BoxMonkey::Const(l + r),
            (l, r) => BoxMonkey::Add(Box::new(l), Box::new(r)),
        },
        BoxMonkey::Sub(left, right) => match (reduce(left), reduce(right)) {
            (BoxMonkey::Const(l), BoxMonkey::Const(r)) => BoxMonkey::Const(l - r),
            (l, r) if l == r => BoxMonkey::Const(0),
            (l, r) => BoxMonkey::Sub(Box::new(l), Box::new(r)),
        },
        BoxMonkey::Mul(left, right) => match (reduce(left), reduce(right)) {
            (BoxMonkey::Const(l), BoxMonkey::Const(r)) => BoxMonkey::Const(l * r),
            (l, r) => BoxMonkey::Mul(Box::new(l), Box::new(r)),
        },
        BoxMonkey::Div(left, right) => match (reduce(left), reduce(right)) {
            (BoxMonkey::Const(l), BoxMonkey::Const(r)) => BoxMonkey::Const(l / r),
            (l, r) => BoxMonkey::Div(Box::new(l), Box::new(r)),
        },
    }
}

fn to_box(monkey: &str, input: &Input) -> BoxMonkey {
    match &input[monkey] {
        Monkey::Const(n) => BoxMonkey::Const(*n),
        Monkey::Human => BoxMonkey::Human,
        Monkey::Add(left, right) => BoxMonkey::Add(
            Box::new(to_box(left, input)),
            Box::new(to_box(right, input)),
        ),
        Monkey::Sub(left, right) => BoxMonkey::Sub(
            Box::new(to_box(left, input)),
            Box::new(to_box(right, input)),
        ),
        Monkey::Mul(left, right) => BoxMonkey::Mul(
            Box::new(to_box(left, input)),
            Box::new(to_box(right, input)),
        ),
        Monkey::Div(left, right) => BoxMonkey::Div(
            Box::new(to_box(left, input)),
            Box::new(to_box(right, input)),
        ),
    }
}

fn solve(left: BoxMonkey, right: BoxMonkey) -> i64 {
    match (reduce(&left), reduce(&right)) {
        (BoxMonkey::Human, BoxMonkey::Const(n)) | (BoxMonkey::Const(n), BoxMonkey::Human) => n,
        (BoxMonkey::Add(l, r), BoxMonkey::Const(r_value)) => match (*l, *r) {
            (BoxMonkey::Const(n), other) | (other, BoxMonkey::Const(n)) => {
                solve(other, BoxMonkey::Const(r_value - n))
            }
            (l, r) => unimplemented!("{:?} {:?}", l, r),
        },
        (BoxMonkey::Sub(l, r), BoxMonkey::Const(r_value)) => match (*l, *r) {
            (other, BoxMonkey::Const(remove)) => solve(other, BoxMonkey::Const(r_value + remove)),
            (BoxMonkey::Const(l), other) => solve(other, BoxMonkey::Const(l - r_value)),
            (l, r) => unimplemented!("{:?} {:?}", l, r),
        },
        (BoxMonkey::Mul(l, r), BoxMonkey::Const(r_value)) => match (*l, *r) {
            (BoxMonkey::Const(n), other) | (other, BoxMonkey::Const(n)) => {
                solve(other, BoxMonkey::Const(r_value / n))
            }
            (l, r) => unimplemented!("{:?} {:?}", l, r),
        },
        (BoxMonkey::Div(l, r), BoxMonkey::Const(r_value)) => match (*l, *r) {
            (l, BoxMonkey::Const(divisor)) => solve(l, BoxMonkey::Const(divisor * r_value)),
            (l, r) => unimplemented!("{:?} {:?}", l, r),
        },
        (l, r) => unimplemented!("{:?} {:?}", l, r),
    }
}

fn two_impl(input: &Input) -> i64 {
    let (left, right) = match &input["root"] {
        Monkey::Const(_) => panic!(),
        Monkey::Human => panic!(),
        Monkey::Add(left, right) => (left, right),
        Monkey::Sub(left, right) => (left, right),
        Monkey::Mul(left, right) => (left, right),
        Monkey::Div(left, right) => (left, right),
    };

    let mut input = input.clone();
    input.insert("humn".to_string(), Monkey::Human);

    let l_box = to_box(left, &input);
    let r_box = to_box(right, &input);

    solve(l_box, r_box)
}

fn parse(reader: Vec<String>) -> Input {
    let mut monkeys = HashMap::new();
    for row in reader {
        let items = row.split(' ').collect::<Vec<_>>();
        let name = items[0].to_string().trim_end_matches(':').to_string();
        if items.len() == 2 {
            monkeys.insert(name, Monkey::Const(items[1].parse().unwrap()));
        } else {
            let left = items[1].to_string();
            let right = items[3].to_string();
            let monkey = match items[2] {
                "+" => Monkey::Add(left, right),
                "-" => Monkey::Sub(left, right),
                "*" => Monkey::Mul(left, right),
                "/" => Monkey::Div(left, right),
                _otherwise => unreachable!(),
            };
            monkeys.insert(name, monkey);
        }
    }
    monkeys
}

pub fn twentyone() -> Result<(), std::io::Error> {
    let file = File::open("21_input").unwrap();
    let reader = BufReader::new(file);
    let input = parse(reader.lines().map(|t| t.unwrap()).collect());
    println!("Day 21 part 1: {}", one_impl(&input));
    println!("Day 21 part 2: {}", two_impl(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_21::{one_impl, parse, two_impl};

    #[test]
    fn it_works() {
        let input = "
root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
"
        .trim()
        .lines()
        .collect::<Vec<_>>();
        let parsed = parse(input.iter().map(|s| s.to_string()).collect());
        assert_eq!(152, one_impl(&parsed));
        assert_eq!(301, two_impl(&parsed));
    }
}
