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
    // horizontally
    let num_rows = block.len();
    'outer: for (idx, a) in generate_combinations(num_rows).into_iter().enumerate() {
        for [b, c] in a {
            let d = &block[b];
            let e = &block[c];

            if d != e {
                continue 'outer;
            }
        }
        return (idx + 1) * 100;
    }

    // vertically
    let num_cols = block[0].len();
    'outer: for (idx, a) in generate_combinations(num_cols).into_iter().enumerate() {
        for [b, c] in a {
            let d = block.iter().map(|line| &line[b]).collect::<Vec<_>>();
            let e = block.iter().map(|line| &line[c]).collect::<Vec<_>>();

            if d != e {
                continue 'outer;
            }
        }
        return idx + 1;
    }

    // ---
    unreachable!()
}

fn generate_combinations(n: usize) -> Vec<Vec<[usize; 2]>> {
    let mut result = Vec::new();

    for i in 1..n {
        let mut inner_vec = Vec::new();
        for j in 0..(i.min(n - i)) {
            inner_vec.push([i - j - 1, j + i]);
        }
        result.push(inner_vec);
    }

    result
}
