use std::collections::HashMap;

const INPUT: &str = include_str!("day08.txt");

fn main() {
    let input = "RL

    AAA = (BBB, CCC)
    BBB = (DDD, EEE)
    CCC = (ZZZ, GGG)
    DDD = (DDD, DDD)
    EEE = (EEE, EEE)
    GGG = (GGG, GGG)
    ZZZ = (ZZZ, ZZZ)";

    let input2 = "LLR

    AAA = (BBB, BBB)
    BBB = (AAA, ZZZ)
    ZZZ = (ZZZ, ZZZ)";

    dbg!(part_1(input));
    dbg!(part_1(input2));
    // dbg!(part_2(input));

    dbg!(part_1(INPUT));
    // dbg!(part_2(INPUT));
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

// fn part_2(input: &str) -> usize {
//     todo!()
// }

fn parse_line(line: &str) -> (&str, [&str; 2]) {
    let (a, b) = line.split_once(" = ").unwrap();
    let l = &b[1..4];
    let r = &b[6..9];
    (a, [l, r])
}
