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
    // assert_eq!(dbg!(part_1(INPUT1)), 405);
    // dbg!(part_1(INPUT));

    assert_eq!(dbg!(part_2(INPUT1)), 400);
    // dbg!(part_2(INPUT));
}

fn part_1(input: &str) -> usize {
    let blocks = parse_input(input);
    // dbg!(&blocks);

    blocks.into_iter().filter_map(process_block).sum()
}

fn part_2(input: &str) -> usize {
    let blocks = parse_input(input);
    // dbg!(&blocks);

    // for each block generate a list of blocks where exactly one token is flipped
    let repaired_blocks = blocks.into_iter().map(|block| {
        let height = block.len();
        let width = block[0].len();
        dbg!(height, width, height * width);
        let mut repaired_blocks = Vec::with_capacity(height * width);
        for b in 0..height {
            for c in 0..width {
                let mut d = block.clone();
                d[b][c].flip();
                repaired_blocks.push(d);
            }
        }
        repaired_blocks
    });

    repaired_blocks
        .into_iter()
        .map(|blocks| blocks.into_iter().find_map(process_block).unwrap())
        .inspect(|result| println!("result={result}\n"))
        .sum()
}

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

impl Token {
    fn flip(&mut self) {
        *self = match self {
            Token::Ash => Token::Rock,
            Token::Rock => Token::Ash,
        }
    }
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

fn process_block(block: Vec<Vec<Token>>) -> Option<usize> {
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
        return Some((idx + 1) * 100);
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
        return Some(idx + 1);
    }

    // ---
    None
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
