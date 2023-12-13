use std::fmt::Debug;

const INPUT: &str = include_str!("day13.txt");

const INPUT1: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

fn main() {
    assert_eq!(dbg!(part_1(INPUT1)), 405);
    // dbg!(part_1(INPUT));

    // assert_eq!(dbg!(part_2(INPUT1)), 525152);
    // dbg!(part_2(INPUT));
}

fn part_1(input: &str) -> usize {
    let blocks = parse_input(input);
    // dbg!(&blocks);

    blocks.into_iter().map(process_block).sum()
}

// fn part_2(input: &str) -> usize {
//     todo!()
// }

fn parse_input(input: &str) -> Vec<Vec<Vec<Token>>> {
    input.split("\n\n").map(parse_block).collect()
}

fn parse_block(block: &str) -> Vec<Vec<Token>> {
    block
        .lines()
        .map(|line| line.chars().map(Into::into).collect())
        .collect()
}

#[derive(Clone, PartialEq)]
enum Token {
    Ash,
    Rock,
}

impl From<char> for Token {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Ash,
            '#' => Self::Rock,
            _ => unreachable!(),
        }
    }
}

impl From<&Token> for char {
    fn from(value: &Token) -> Self {
        match value {
            Token::Ash => '.',
            Token::Rock => '#',
        }
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<char>::into(self))
    }
}

fn process_block(block: Vec<Vec<Token>>) -> usize {
    todo!()
}
