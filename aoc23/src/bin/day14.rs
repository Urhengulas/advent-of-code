use std::fmt::Debug;

const INPUT: &str = include_str!("day14.txt");

const INPUT1: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

fn main() {
    assert_eq!(dbg!(part_1(INPUT1)), 405);
    // dbg!(part_1(INPUT));

    // assert_eq!(dbg!(part_2(INPUT1)), 400);
    // dbg!(part_2(INPUT));
}

fn part_1(input: &str) -> usize {
    let a = parse_input(input);
    dbg!(&a);

    todo!()
}

// fn part_2(input: &str) -> usize {
//     todo!()
// }

fn parse_input(input: &str) -> Vec<Vec<Token>> {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Vec<Token> {
    line.chars().map(Into::into).collect()
}

#[derive(Clone, Debug)]
enum Token {
    Round,
    Cube,
    Empty,
}

impl From<char> for Token {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '#' => Self::Cube,
            'O' => Self::Round,
            _ => unreachable!(),
        }
    }
}
