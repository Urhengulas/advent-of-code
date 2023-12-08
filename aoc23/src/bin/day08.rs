const INPUT: &str = include_str!("day08.txt");

fn main() {
    let input = "";

    dbg!(part_1(input));
    // dbg!(part_2(input));

    // dbg!(part_1(INPUT));
    // dbg!(part_2(INPUT));
}

fn part_1(input: &str) -> usize {
    let a = input
        .lines()
        .map(str::trim)
        .map(parse_line)
        .collect::<Vec<_>>();
    dbg!(&a);

    todo!()
}

// fn part_2(input: &str) -> usize {
//     todo!()
// }

fn parse_line(line: &str) -> u32 {
    todo!()
}
