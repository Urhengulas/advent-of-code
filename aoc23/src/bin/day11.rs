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
    // dbg!(part_1(INPUT1));
    // dbg!(part_1(INPUT));

    // dbg!(part_2(INPUT1));
    dbg!(part_2(INPUT));
}

fn part_1(input: &str) -> usize {
    let galaxy = parse_input(input);
    println!("parse_input");
    // dbg!(&galaxy);

    let expanded_galaxy = expand(galaxy);
    println!("expand");
    // dbg!(&expanded_galaxy);

    let positions = positions(&expanded_galaxy);
    println!("positions");
    // dbg!(&positions);

    let distances = distances(&positions);
    println!("distances");
    // dbg!(&distances);

    distances.into_iter().sum()
}

fn part_2(input: &str) -> usize {
    let galaxy = parse_input(input);
    // dbg!(&galaxy);

    let empty = detect_empty(&galaxy);
    // dbg!(&empty);

    let positions = positions(&galaxy);
    // dbg!(&positions);

    let distances = distances2(&positions, &empty);
    // dbg!(&distances);

    distances.into_iter().sum()
}

#[derive(Clone, Debug)]
enum Token {
    Empty,
    Galaxy,
}

impl From<char> for Token {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '#' => Self::Galaxy,
            _ => unreachable!(),
        }
    }
}

type Universe = Vec<Vec<Token>>;

fn parse_input(input: &str) -> Universe {
    input
        .lines()
        .map(str::trim)
        .map(|line| line.chars().map(Into::into).collect())
        .collect()
}

fn expand(mut universe: Universe) -> Universe {
    // rows
    let mut row_idx = 0;
    while row_idx < universe.len() {
        let row = &universe[row_idx];
        if row.iter().all(|a| matches!(a, Token::Empty)) {
            universe.insert(row_idx, row.clone());
            row_idx += 2;
        } else {
            row_idx += 1;
        }
    }

    // columns
    let mut col_idx = 0;
    while col_idx < universe[0].len() {
        let mut col = universe.iter().map(|row| &row[col_idx]);
        if col.all(|a| matches!(a, Token::Empty)) {
            for row in &mut universe {
                row.insert(col_idx, Token::Empty)
            }
            col_idx += 2;
        } else {
            col_idx += 1;
        }
    }

    // ---
    universe
}

type Pos = [usize; 2];

fn positions(universe: &Universe) -> Vec<Pos> {
    let mut positions = Vec::new();
    for (row_idx, row) in universe.iter().enumerate() {
        for (col_idx, token) in row.iter().enumerate() {
            if matches!(token, Token::Galaxy) {
                positions.push([row_idx, col_idx]);
            }
        }
    }
    positions
}

fn distances(positions: &Vec<Pos>) -> Vec<usize> {
    let mut distances = Vec::new();
    let mut seen_positions = Vec::new();
    for (outer_idx, outer_pos) in positions.iter().enumerate() {
        for (inner_idx, inner_pos) in positions.iter().enumerate() {
            if outer_idx == inner_idx {
                continue; // same galaxy
            } else if seen_positions
                .iter()
                .any(|a| *a == [outer_idx, inner_idx] || *a == [inner_idx, outer_idx])
            {
                continue; // already processed distance
            } else {
                let d_row = outer_pos[0]
                    .checked_sub(inner_pos[0])
                    .unwrap_or_else(|| inner_pos[0].checked_sub(outer_pos[0]).unwrap());
                let d_col = outer_pos[1]
                    .checked_sub(inner_pos[1])
                    .unwrap_or_else(|| inner_pos[1].checked_sub(outer_pos[1]).unwrap());
                let dist = d_row + d_col;
                distances.push(dist);
                seen_positions.push([outer_idx, inner_idx]);
            }
        }
    }
    distances
}

fn detect_empty(universe: &Universe) -> [Vec<usize>; 2] {
    let mut empty_rows = Vec::new();
    for (row_idx, row) in universe.iter().enumerate() {
        if row.iter().all(|token| matches!(token, Token::Empty)) {
            empty_rows.push(row_idx)
        }
    }

    let mut empty_cols = Vec::new();
    for col_idx in 0..universe[0].len() {
        let mut col = universe.iter().map(|row| &row[col_idx]);
        if col.all(|token| matches!(token, Token::Empty)) {
            empty_cols.push(col_idx)
        }
    }

    [empty_rows, empty_cols]
}

fn distances2(positions: &Vec<Pos>, empty: &[Vec<usize>; 2]) -> Vec<usize> {
    let [empty_rows, empty_cols] = empty;

    let mut distances = Vec::new();
    let mut seen_positions = Vec::new();
    for (outer_idx, outer_pos) in positions.iter().enumerate() {
        for (inner_idx, inner_pos) in positions.iter().enumerate() {
            if outer_idx == inner_idx {
                continue; // same galaxy
            } else if seen_positions
                .iter()
                .any(|a| *a == [outer_idx, inner_idx] || *a == [inner_idx, outer_idx])
            {
                continue; // already processed distance
            } else {
                const EXPANSION: usize = 1_000_000;
                let mut dist = 0;

                let row_range = if outer_pos[0] < inner_pos[0] {
                    outer_pos[0]..inner_pos[0]
                } else {
                    inner_pos[0]..outer_pos[0]
                };
                for i in row_range {
                    if empty_rows.contains(&i) {
                        dist += EXPANSION
                    } else {
                        dist += 1
                    }
                }

                let col_range = if outer_pos[1] < inner_pos[1] {
                    outer_pos[1]..inner_pos[1]
                } else {
                    inner_pos[1]..outer_pos[1]
                };
                for i in col_range {
                    if empty_cols.contains(&i) {
                        dist += EXPANSION
                    } else {
                        dist += 1
                    }
                }

                distances.push(dist);
                seen_positions.push([outer_idx, inner_idx]);
            }
        }
    }
    distances
}
