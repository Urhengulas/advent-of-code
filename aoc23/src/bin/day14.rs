use std::fmt::{self, Debug};

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

const INPUT2: &str = "OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....";

fn main() {
    assert_eq!(dbg!(part_1(INPUT1)), 136);
    // dbg!(part_1(INPUT));

    // assert_eq!(dbg!(part_2(INPUT1)), 400);
    // dbg!(part_2(INPUT));
}

fn part_1(input: &str) -> usize {
    let map = parse_input(input);
    // dbg!(&map);

    let north_map = slide_north(map);
    // dbg!(&north_map);

    for row in north_map {
        for token in row {
            print!("{token}")
        }
        println!()
    }

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

#[derive(Clone, Debug, PartialEq)]
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

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Empty => '.',
                Self::Cube => '#',
                Self::Round => 'O',
            }
        )
    }
}

fn slide_north(mut map: Vec<Vec<Token>>) -> Vec<Vec<Token>> {
    let num_rows = map.len();
    let num_cols = map[0].len();
    // dbg!(num_rows, num_cols);

    for col_idx in 0..num_cols {
        let mut col = map
            .iter_mut()
            .map(|line| &mut line[col_idx])
            .collect::<Vec<_>>();
        // dbg!(&col);

        // 1. find cube and next cube
        // 2. count round in-between cubes
        // 3. write round below upper cube and empty until next cube
        // 4. set cube to next cube and search next next cube
        // 5. goto 2

        let mut a = 0;
        loop {
            let b = match col[a..].iter().position(|token| **token == Token::Cube) {
                Some(idx) => a + idx,
                None => num_rows,
            };
            // dbg!(b);
            // dbg!(a);

            let c = &mut col[a..b];
            // dbg!(&c);

            let d = c.iter().filter(|token| ***token == Token::Round).count();
            // dbg!(d);

            for (e, f) in c.iter_mut().enumerate() {
                **f = if e < d { Token::Round } else { Token::Empty }
            }
            // dbg!(&c);

            if b == num_rows {
                break;
            } else if a == b {
                a += 1;
                continue;
            } else {
                a = b;
            }
        }
        // dbg!(col);
    }

    map
}
