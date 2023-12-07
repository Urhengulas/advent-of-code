const INPUT: &str = include_str!("day07.txt");

fn main() {
    let input = "";

    dbg!(part_1(input));
    // dbg!(part_2(input));

    // dbg!(part_1(INPUT));
    // dbg!(part_2(INPUT));
}

fn part_1(input: &str) -> u64 {
    let a = input
        .lines()
        .map(str::trim)
        .map(parse_line)
        .collect::<Vec<_>>();
    dbg!(&a);

    todo!()
}

// fn part_2(input: &str) -> u64 {
//     todo!()
// }

fn parse_line(line: &str) -> Vec<u64> {
    line.split(' ')
        .map(str::trim)
        .filter_map(|b| b.parse().ok())
        .collect()
}
