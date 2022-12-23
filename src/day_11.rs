use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

struct Monkey {
    operation: Box<dyn Fn(i64) -> i64>,
    test: i64,
    targets: (usize, usize),
}

type Input = (Vec<Vec<i64>>, Vec<Monkey>);

fn one_impl((its, monkeys): &Input) -> usize {
    let mut items = its.clone();
    let mut counts = monkeys.iter().map(|_| 0).collect::<Vec<usize>>();
    for _ in 0..20 {
        for (i, monkey) in monkeys.iter().enumerate() {
            let m_items = items[i].clone();
            items[i] = Vec::new();
            for item in m_items {
                counts[i] += 1;
                let n = (monkey.operation)(item) / 3;
                let target = if n % monkey.test == 0 {
                    monkey.targets.0
                } else {
                    monkey.targets.1
                };
                items[target].push(n);
            }
        }
    }
    counts.sort();
    counts[counts.len() - 1] * counts[counts.len() - 2]
}

fn two_impl((its, monkeys): &Input) -> usize {
    let mul: i64 = monkeys.iter().map(|m| m.test).product();
    let mut items = its.clone();
    let mut counts = monkeys.iter().map(|_| 0).collect::<Vec<usize>>();
    for _ in 0..10000 {
        for (i, monkey) in monkeys.iter().enumerate() {
            let m_items = items[i].clone();
            items[i] = Vec::new();
            for item in m_items {
                counts[i] += 1;
                let n = (monkey.operation)(item) % mul;
                let target = if n % monkey.test == 0 {
                    monkey.targets.0
                } else {
                    monkey.targets.1
                };
                items[target].push(n);
            }
        }
    }
    counts.sort();
    counts[counts.len() - 1] * counts[counts.len() - 2]
}

fn parse(reader: Vec<String>) -> Input {
    let mut monkeys = Vec::new();
    let mut starting_items = Vec::new();
    for i in 0..=reader.len() / 7 {
        starting_items.push(
            reader[(i * 7) + 1]
                .split(": ")
                .last()
                .unwrap()
                .split(", ")
                .map(|n| n.parse().unwrap())
                .collect::<Vec<i64>>(),
        );
        let op_parts = reader[(i * 7) + 2]
            .split("new = old ")
            .last()
            .unwrap()
            .split(' ')
            .collect::<Vec<_>>();
        let operation = if op_parts[1] == "old" {
            Box::new(move |n| n * n) as Box<dyn Fn(i64) -> i64>
        } else {
            let op_num = op_parts[1].parse::<i64>().unwrap();
            match op_parts[0] {
                "*" => Box::new(move |n| n * op_num) as Box<dyn Fn(i64) -> i64>,
                "+" => Box::new(move |n| n + op_num) as Box<dyn Fn(i64) -> i64>,
                other => unimplemented!("{}", other),
            }
        };
        let test_op_num = reader[(i * 7) + 3]
            .split("divisible by ")
            .last()
            .unwrap()
            .parse::<i64>()
            .unwrap();
        let test = test_op_num;
        let if_t = reader[(i * 7) + 4]
            .split("throw to monkey ")
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let if_f = reader[(i * 7) + 5]
            .split("throw to monkey ")
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        monkeys.push(Monkey {
            operation,
            test,
            targets: (if_t, if_f),
        });
    }
    (starting_items, monkeys)
}

pub fn eleven() -> Result<(), std::io::Error> {
    let file = File::open("11_input")?;
    let reader = BufReader::new(file);
    let input = parse(reader.lines().map(|t| t.unwrap()).collect());
    println!("Day 11 part 1: {}", one_impl(&input));
    println!("Day 11 part 2: {}", two_impl(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_11::{one_impl, parse, two_impl};

    #[test]
    fn it_works() {
        let input = "
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
"
        .trim()
        .lines()
        .collect::<Vec<_>>();
        let parsed = parse(input.iter().map(|s| s.to_string()).collect());
        assert_eq!(10605, one_impl(&parsed));
        assert_eq!(2713310158, two_impl(&parsed));
    }
}
