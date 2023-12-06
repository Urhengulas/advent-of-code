const INPUT: &str = include_str!("day06.txt");

fn main() {
    let input = "";

    dbg!(part_1(input));
    // dbg!(part_2(input));

    // dbg!(part_1(INPUT));
    // dbg!(part_2(INPUT));
}

fn part_1(input: &str) -> u32 {
    input.lines().map(str::trim).map(parse_line).sum()
}

// fn part_2(input: &str) -> u32 {
//     todo!()
// }

fn parse_line(line: &str) -> u32 {
    todo!()
}
