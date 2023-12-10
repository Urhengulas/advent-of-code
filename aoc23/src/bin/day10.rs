const INPUT: &str = include_str!("day10.txt");

const INPUT1: &str = "";

fn main() {
    dbg!(part_1(INPUT1));
    dbg!(part_2(INPUT1));

    dbg!(part_1(INPUT));
    dbg!(part_2(INPUT));
}

fn part_1(input: &str) -> i64 {
    let a = parse_input(input);
    // dbg!(&a);

    a.into_iter().map(|b| 0).sum()
}

fn part_2(input: &str) -> i64 {
    todo!()
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(str::trim)
        .map(|line| {
            line.split(' ')
                .map(|a| a.parse().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}
