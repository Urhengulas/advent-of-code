use std::collections::HashMap;

const INPUT: &str = include_str!("day08.txt");

const INPUT1: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

const INPUT2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

const INPUT3: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

fn main() {
    dbg!(part_1(INPUT1));
    dbg!(part_1(INPUT2));
    dbg!(part_2(INPUT3));

    dbg!(part_1(INPUT));
    dbg!(part_2(INPUT));
}

fn part_1(input: &str) -> usize {
    let (a, b) = input.split_once("\n\n").unwrap();
    // dbg!(a);
    // dbg!(b);

    let c = b
        .lines()
        .map(str::trim)
        .map(parse_line)
        .collect::<HashMap<_, _>>();
    // dbg!(&c);

    let mut location = "AAA";
    let mut step_count = 0;
    for d in a.chars().cycle() {
        if location == "ZZZ" {
            break;
        }

        let e = match d {
            'L' => 0,
            'R' => 1,
            _ => unreachable!(),
        };
        let f = c.get(location).unwrap();
        let g = f[e];
        location = g;

        step_count += 1;
    }

    step_count
}

fn part_2(input: &str) -> u64 {
    let (a, b) = input.split_once("\n\n").unwrap();
    // dbg!(a);
    // dbg!(b);

    let c = b
        .lines()
        .map(str::trim)
        .map(parse_line)
        .collect::<HashMap<_, _>>();
    // dbg!(&c);

    let locations = c
        .keys()
        .filter(|key| key.ends_with('A'))
        .copied()
        .collect::<Vec<_>>();
    // dbg!(&locations);

    let counts = locations
        .iter()
        .map(|location| {
            let mut location = *location;
            let mut step_count = 0;
            for d in a.chars().cycle() {
                if location.ends_with('Z') {
                    break;
                }

                let e = match d {
                    'L' => 0,
                    'R' => 1,
                    _ => unreachable!(),
                };
                let f = c.get(location).unwrap();
                let g = f[e];
                location = g;

                step_count += 1;
            }
            step_count
        })
        .collect::<Vec<_>>();
    // dbg!(&counts);

    let step_count = counts
        .into_iter()
        .reduce(|acc, e| num::integer::lcm(acc, e))
        .unwrap();

    step_count
}

fn parse_line(line: &str) -> (&str, [&str; 2]) {
    let (a, b) = line.split_once(" = ").unwrap();
    let l = &b[1..4];
    let r = &b[6..9];
    (a, [l, r])
}
