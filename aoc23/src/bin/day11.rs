const INPUT: &str = include_str!("day11.txt");

const INPUT1: &str = "";

fn main() {
    dbg!(part_1(INPUT1));
    // dbg!(part_1(INPUT));

    // dbg!(part_2(INPUT1));
    // dbg!(part_2(INPUT));
}

fn part_1(input: &str) -> usize {
    let a = parse_input(input);
    // dbg!(&a);

    todo!()
}

// fn part_2(input: &str) -> u32 {
//     todo!()
// }

fn parse_input(input: &str) -> Vec<u32> {
    input.lines().map(str::trim).map(|line| todo!()).collect()
}
