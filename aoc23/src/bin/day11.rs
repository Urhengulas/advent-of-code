const INPUT: &str = include_str!("day11.txt");

const INPUT1: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

fn main() {
    dbg!(part_1(INPUT1));
    // dbg!(part_1(INPUT));

    // dbg!(part_2(INPUT1));
    // dbg!(part_2(INPUT));
}

fn part_1(input: &str) -> usize {
    let galaxy = parse_input(input);
    // dbg!(&galaxy);

    let expanded_galaxy = expand(galaxy);
    dbg!(&expanded_galaxy);

    let positions = positions(&expanded_galaxy);
    dbg!(&positions);

    todo!()
}

// fn part_2(input: &str) -> u32 {
//     todo!()
// }

#[derive(Clone, Debug)]
enum Token {
    Empty,
    Galaxy(u32),
}

type Galaxy = Vec<Vec<Token>>;

fn parse_input(input: &str) -> Galaxy {
    let mut galaxy_id = 0;
    input
        .lines()
        .map(str::trim)
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Token::Empty,
                    '#' => {
                        galaxy_id += 1;
                        Token::Galaxy(galaxy_id)
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

fn expand(mut galaxy: Galaxy) -> Galaxy {
    // rows
    let mut row_idx = 0;
    while row_idx < galaxy.len() {
        let row = &galaxy[row_idx];
        if row.iter().all(|a| matches!(a, Token::Empty)) {
            galaxy.insert(row_idx, row.clone());
            row_idx += 2;
        } else {
            row_idx += 1;
        }
    }

    // columns
    let mut col_idx = 0;
    while col_idx < galaxy[0].len() {
        let mut col = galaxy.iter().map(|row| &row[col_idx]);
        if col.all(|a| matches!(a, Token::Empty)) {
            for row in &mut galaxy {
                row.insert(col_idx, Token::Empty)
            }
            col_idx += 2;
        } else {
            col_idx += 1;
        }
    }

    // ---
    galaxy
}

type Pos = [usize; 2];

fn positions(galaxy: &Galaxy) -> Vec<Pos> {
    let mut positions = Vec::new();
    for (row_idx, row) in galaxy.iter().enumerate() {
        for (col_idx, token) in row.iter().enumerate() {
            if matches!(token, Token::Galaxy(_)) {
                positions.push([row_idx, col_idx]);
            }
        }
    }
    positions
}
