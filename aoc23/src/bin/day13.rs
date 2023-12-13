const INPUT: &str = include_str!("day13.txt");

const INPUT1: &str = "";

fn main() {
    assert_eq!(dbg!(part_1(INPUT1)), 21);
    // dbg!(part_1(INPUT));

    // assert_eq!(dbg!(part_2(INPUT1)), 525152);
    // dbg!(part_2(INPUT));
}

fn part_1(input: &str) -> usize {
    let lines = parse_input(input);
    // dbg!(&a);

    lines.into_iter().map(|line| line.into_iter().sum::<usize>()).sum()
}

// fn part_2(input: &str) -> usize {
//     todo!()
// }

fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Vec<usize> {
    todo!()
}
