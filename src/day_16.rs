use itertools::Itertools;
use std::collections::{BTreeSet, BinaryHeap, HashMap};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use scanf::sscanf;

type Input = (u8, HashMap<u8, (usize, Vec<u8>)>, HashMap<u8, String>);

fn contains(set: usize, bit: u8) -> bool {
    (set & (1 << bit)) != 0
}

fn set(set: usize, bit: u8) -> usize {
    let next = set | (1 << (bit as usize));
    assert!(set != next);
    next
}

fn one_impl(input: &Input) -> usize {
    let routes = &input.1;
    let mut queue: BinaryHeap<(usize, usize, u8, usize)> = BinaryHeap::new();

    let mut best_flow = 0;

    let pos_route_count = input.1.iter().filter(|(_, (flow, _))| *flow > 0).count() as u8;
    let fw_route_count = input.1.len() + 1;
    let mut fw_routes: Vec<Vec<usize>> = (0..fw_route_count)
        .map(|_| (0..fw_route_count).map(|_| usize::MAX).collect())
        .collect();

    for (key, (_flow, routes)) in &input.1 {
        let k = *key as usize;
        fw_routes[k][k] = 0;
        for route in routes {
            let r = *route as usize;
            fw_routes[k][r] = 1;
            fw_routes[r][k] = 1;
        }
    }

    for k in 0..fw_route_count {
        for i in 0..fw_route_count {
            for j in 0..fw_route_count {
                if fw_routes[i][j] > fw_routes[i][k].saturating_add(fw_routes[k][j]) {
                    fw_routes[i][j] = fw_routes[i][k].saturating_add(fw_routes[k][j])
                }
            }
        }
    }

    for target in 1..=pos_route_count {
        let route_time = fw_routes[input.0 as usize][target as usize];
        let new_time = 30usize.saturating_sub(route_time);
        queue.push((new_time, 0, target, 0));
    }

    while let Some((mut time, flow, cur, visited)) = queue.pop() {
        time -= 1;
        let flow_increase = time * routes[&cur].0;
        let new_flow = flow_increase + flow;
        let new_visited = set(visited, cur);
        best_flow = best_flow.max(new_flow);

        for target in 1..=pos_route_count {
            let route_time = fw_routes[cur as usize][target as usize];
            if !contains(new_visited, target) && time > route_time {
                let new_time = time - route_time;
                queue.push((new_time, new_flow, target, new_visited));
            }
        }
    }

    best_flow
}

fn two_impl(input: &Input) -> usize {
    type Unit = (u8, usize);
    dbg!(&input);
    let routes = &input.1;
    let mut queue: BTreeSet<(usize, usize, Unit, Option<Unit>, usize)> = BTreeSet::new();

    let mut best_flow = 0;

    let pos_route_count = input.1.iter().filter(|(_, (flow, _))| *flow > 0).count() as u8;
    let fw_route_count = input.1.len() + 1;
    let mut fw_routes: Vec<Vec<usize>> = (0..fw_route_count)
        .map(|_| (0..fw_route_count).map(|_| usize::MAX).collect())
        .collect();

    for (key, (_flow, routes)) in &input.1 {
        let k = *key as usize;
        fw_routes[k][k] = 0;
        for route in routes {
            let r = *route as usize;
            fw_routes[k][r] = 1;
            fw_routes[r][k] = 1;
        }
    }

    for k in 0..fw_route_count {
        for i in 0..fw_route_count {
            for j in 0..fw_route_count {
                if fw_routes[i][j] > fw_routes[i][k].saturating_add(fw_routes[k][j]) {
                    fw_routes[i][j] = fw_routes[i][k].saturating_add(fw_routes[k][j])
                }
            }
        }
    }

    for elephant in 1..=pos_route_count {
        for target in elephant..=pos_route_count {
            if target != elephant {
                let route_time = fw_routes[input.0 as usize][target as usize];
                let elephant_time = fw_routes[input.0 as usize][elephant as usize];
                let visited = set(set(0, target), elephant);
                let min_time = route_time.min(elephant_time);
                let new_time = 26usize.saturating_sub(min_time);
                queue.insert((
                    new_time,
                    0,
                    (target, route_time - min_time),
                    Some((elephant, elephant_time - min_time)),
                    visited,
                ));
            }
        }
    }

    while let Some((mut time, mut flow, (u1_pos, mut u1_time), mut unit2, visited)) =
        queue.pop_first()
    {
        time -= 1;
        if time == 0 {
            continue;
        }
        if u1_time == 0 {
            flow += time * routes[&u1_pos].0;
        }
        if let Some((u2_pos, 0)) = unit2 {
            flow += time * routes[&u2_pos].0;
        }
        best_flow = best_flow.max(flow);

        if let (0, Some((u2_pos, 0))) = (u1_time, unit2) {
            for u1_target in 1..=pos_route_count {
                for u2_target in 1..=pos_route_count {
                    if u1_target != u2_target {
                        let u1_time = fw_routes[u1_pos as usize][u1_target as usize];
                        let u2_time = fw_routes[u2_pos as usize][u2_target as usize];
                        if !contains(visited, u1_target)
                            && time > u1_time
                            && !contains(visited, u2_target)
                            && time > u2_time
                        {
                            let new_visited = set(set(visited, u1_target), u2_target);
                            let min_time = u1_time.min(u2_time);
                            queue.insert((
                                time - min_time,
                                flow,
                                (u1_target, u1_time - min_time),
                                Some((u2_target, u2_time - min_time)),
                                new_visited,
                            ));
                        }
                        if !contains(visited, u1_target) && time > u1_time {
                            queue.insert((
                                time - u1_time,
                                flow,
                                (u1_target, 0),
                                None,
                                set(visited, u1_target),
                            ));
                        }
                        if !contains(visited, u2_target) && time > u2_time {
                            queue.insert((
                                time - u2_time,
                                flow,
                                (u2_target, 0),
                                None,
                                set(visited, u2_target),
                            ));
                        }
                    }
                }
            }
        } else if u1_time == 0 {
            unit2 = unit2.map(|(p, t)| (p, t - 1));
            for u1_target in 1..=pos_route_count {
                let u1_time = fw_routes[u1_pos as usize][u1_target as usize];
                if !contains(visited, u1_target) && time > u1_time {
                    let new_visited = set(visited, u1_target);
                    let min_time = u1_time.min(unit2.map(|(_, t)| t).unwrap_or(usize::MAX));
                    queue.insert((
                        time - min_time,
                        flow,
                        (u1_target, u1_time - min_time),
                        unit2.map(|(p, t)| (p, t - min_time)),
                        new_visited,
                    ));
                }
            }
            if let Some((u2_pos, u2_time)) = unit2 {
                queue.insert((time - u2_time, flow, (u2_pos, 0), None, visited));
            }
        } else if let Some((u2_pos, 0)) = unit2 {
            u1_time -= 1;
            for u2_target in 1..=pos_route_count {
                let u2_time = fw_routes[u2_pos as usize][u2_target as usize];
                if !contains(visited, u2_target) && time > u2_time {
                    let new_visited = set(visited, u2_target);
                    let min_time = u2_time.min(u1_time);
                    queue.insert((
                        time - min_time,
                        flow,
                        (u1_pos, u1_time - min_time),
                        Some((u2_target, u2_time - min_time)),
                        new_visited,
                    ));
                }
            }
            queue.insert((time - u1_time, flow, (u1_pos, 0), None, visited));
        } else {
            unreachable!()
        }
    }

    best_flow
}

fn parse(reader: Vec<String>) -> Input {
    let mut valves = HashMap::new();
    let mut next_index = 0;
    let mut translation: HashMap<String, u8> = HashMap::new();
    let mut flow_rates: HashMap<String, usize> = HashMap::new();
    let mut tunnels: HashMap<String, Vec<String>> = HashMap::new();
    for row in reader {
        let mut name: String = "".to_string();
        let mut s: String = "".to_string();
        let mut s2: String = "".to_string();
        let mut flowrate: usize = 0;
        let mut others: String = "".to_string();
        sscanf!(
            &row,
            "Valve {} has flow rate={}; tunnel{} lead{} to valve{}",
            name,
            flowrate,
            s,
            s2,
            others,
        )
        .unwrap();

        let routes = others
            .split_once(' ')
            .map(|(_, s)| s)
            .unwrap_or(&others)
            .split(", ")
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        tunnels.insert(name.clone(), routes.clone());
        flow_rates.insert(name, flowrate);
    }

    for (route, flow_rate) in flow_rates.iter().sorted() {
        if *flow_rate > 0 {
            translation.entry(route.clone()).or_insert_with(|| {
                next_index += 1;
                next_index
            });
        }
    }
    for (route, _) in flow_rates.iter().sorted() {
        translation.entry(route.clone()).or_insert_with(|| {
            next_index += 1;
            next_index
        });
    }
    for (route, flow_rate) in &flow_rates {
        valves.insert(
            translation[route],
            (
                *flow_rate,
                tunnels[route]
                    .iter()
                    .map(|route| translation[route])
                    .collect(),
            ),
        );
    }

    (
        translation["AA"],
        valves,
        translation.into_iter().map(|(a, b)| (b, a)).collect(),
    )
}

pub fn sixteen() -> Result<(), std::io::Error> {
    let file = File::open("16_input").unwrap();
    let reader = BufReader::new(file);
    let input = parse(reader.lines().map(|t| t.unwrap()).collect());
    println!("Day 16 part 1: {}", one_impl(&input));
    println!("Day 16 part 2: {}", two_impl(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_16::{one_impl, parse, two_impl};

    #[test]
    fn it_works() {
        let input = "
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
"
        .trim()
        .lines()
        .collect::<Vec<_>>();
        let parsed = parse(input.iter().map(|s| s.to_string()).collect());
        assert_eq!(1651, one_impl(&parsed));
        assert_eq!(1707, two_impl(&parsed));
    }
}
