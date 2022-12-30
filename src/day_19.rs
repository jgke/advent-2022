use scanf::sscanf;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

type QItem = (usize, Rpm, Resources);
type Rpm = (usize, usize, usize, usize);
type Resources = (usize, usize, usize, usize);
type Blueprint = (usize, usize, usize, (usize, usize), (usize, usize));
type Input = Vec<Blueprint>;

fn div_ceil(lhs: usize, rhs: usize) -> usize {
    if lhs % rhs == 0 {
        lhs / rhs
    } else {
        (lhs / rhs) + 1
    }
}

fn do_insert(
    _blueprint: Blueprint,
    max_time: usize,
    queue: &mut BTreeSet<QItem>,
    (time, rpm, res): QItem,
    unit: Resources,
    cost: Resources,
) {
    let calc_time = |cost: usize, cur: usize, rpm: usize| {
        if cost <= cur {
            0
        } else {
            div_ceil(cost - cur, rpm)
        }
    };

    if (rpm.0 == 0 && cost.0 > 0) || (rpm.1 == 0 && cost.1 > 0) || (rpm.2 == 0 && cost.2 > 0) {
        return;
    }

    let t1 = calc_time(cost.0, res.0, rpm.0);
    let t2 = calc_time(cost.1, res.1, rpm.1);
    let t3 = calc_time(cost.2, res.2, rpm.2);
    let t = t1.max(t2).max(t3) + 1;

    if time + t > max_time {
        return;
    }

    let want_item = (
        time + t,
        (
            rpm.0 + unit.0,
            rpm.1 + unit.1,
            rpm.2 + unit.2,
            rpm.3 + unit.3,
        ),
        (
            res.0 + t * rpm.0 - cost.0,
            res.1 + t * rpm.1 - cost.1,
            res.2 + t * rpm.2 - cost.2,
            res.3 + t * rpm.3,
        ),
    );

    queue.insert(want_item);
}

fn simulate(bp: Blueprint, max_time: usize) -> usize {
    let mut queue: BTreeSet<QItem> = BTreeSet::new();
    queue.insert((0, (1, 0, 0, 0), (0, 0, 0, 0)));

    let mut best = 0;

    while let Some(item) = queue.pop_first() {
        best = item.2 .3.max(best);
        if item.0 == max_time {
            continue;
        }

        if item.1 .0 < bp.4 .0 || item.1 .2 < bp.4 .1 {
            if (item.1 .0 < bp.1 || item.1 .0 < bp.2 || item.1 .0 < bp.3 .0 || item.1 .0 < bp.4 .0)
                && item.0 < max_time - 3
            {
                do_insert(
                    bp,
                    max_time,
                    &mut queue,
                    item,
                    (1, 0, 0, 0),
                    (bp.1, 0, 0, 0),
                );
            }

            if (item.1 .2 < bp.3 .1) && item.0 < max_time - 3 {
                do_insert(
                    bp,
                    max_time,
                    &mut queue,
                    item,
                    (0, 1, 0, 0),
                    (bp.2, 0, 0, 0),
                );
            }

            if (item.1 .3 < bp.4 .1) && item.0 < max_time - 2 {
                do_insert(
                    bp,
                    max_time,
                    &mut queue,
                    item,
                    (0, 0, 1, 0),
                    (bp.3 .0, bp.3 .1, 0, 0),
                );
            }
        } else {
            let mut rpm = item.1 .3;
            let mut geodes = item.2 .3;
            for _ in item.0..max_time {
                geodes += rpm;
                rpm += 1;
            }
            queue.insert((max_time, (0, 0, 0, 0), (0, 0, 0, geodes)));
            continue;
        }
        do_insert(
            bp,
            max_time,
            &mut queue,
            item,
            (0, 0, 0, 1),
            (bp.4 .0, 0, bp.4 .1, 0),
        );
        queue.insert((
            max_time,
            (0, 0, 0, 0),
            (0, 0, 0, item.2 .3 + item.1 .3 * (max_time - item.0)),
        ));
    }

    best
}

fn one_impl(input: &Input) -> usize {
    input
        .iter()
        .copied()
        .map(|blueprint| simulate(blueprint, 24) * blueprint.0)
        .sum()
}

fn two_impl(input: &Input) -> usize {
    simulate(input[0], 32) * simulate(input[1], 32) * simulate(input[2], 32)
}

fn parse(reader: Vec<String>) -> Input {
    reader.iter().map(|row| {
        let mut id: usize = 0;
        let mut ore_robot: usize = 0;
        let mut clay_robot: usize = 0;
        let mut obsidian_robot_ore: usize = 0;
        let mut obsidian_robot_clay: usize = 0;
        let mut geode_robot_ore: usize = 0;
        let mut geode_robot_obsidian: usize = 0;

        sscanf!(row,
              "Blueprint {}: Each ore robot costs {} ore. Each clay robot costs {} ore. Each obsidian robot costs {} ore and {} clay. Each geode robot costs {} ore and {} obsidian.", id, ore_robot, clay_robot, obsidian_robot_ore, obsidian_robot_clay, geode_robot_ore, geode_robot_obsidian
                ).unwrap();
        (id, ore_robot, clay_robot, (obsidian_robot_ore, obsidian_robot_clay), (geode_robot_ore, geode_robot_obsidian))
    }).collect()
}

pub fn nineteen() -> Result<(), std::io::Error> {
    let file = File::open("19_input").unwrap();
    let reader = BufReader::new(file);
    let input = parse(reader.lines().map(|t| t.unwrap()).collect());
    println!("Day 19 part 1: {}", one_impl(&input));
    println!("Day 19 part 2: {}", two_impl(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_19::{parse, simulate};
    //use crate::day_19::{one_impl, parse, simulate, two_impl};

    #[test]
    fn it_works() {
        let input = "
Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
"
        .trim()
        .lines()
        .collect::<Vec<_>>();
        let parsed = parse(input.iter().map(|s| s.to_string()).collect());
        assert_eq!(9, simulate(parsed[0], 24));
        //assert_eq!(12, simulate(parsed[1], 24));
        //assert_eq!(33, one_impl(&parsed));
        //assert_eq!(56, simulate(parsed[0], 32));
        //assert_eq!(62, simulate(parsed[1], 32));
    }
}
