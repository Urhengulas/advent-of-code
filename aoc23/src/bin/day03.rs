const INPUT: &str = include_str!("day03.txt");

fn main() {
    let input = "";

    dbg!(part_1(input));
    // dbg!(part_2(input));

    // dbg!(part_1(INPUT));
    // dbg!(part_2(INPUT));
}

fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(str::trim)
        .map(parse_line)
        // .inspect(|a| {
        //     dbg!(a);
        // })
        .sum()
}

fn parse_line(s: &str) -> u32 {
    let s = &s[1..];

    todo!()
}

// fn part_2(input: &str) -> u32 {
//     todo!()
// }
