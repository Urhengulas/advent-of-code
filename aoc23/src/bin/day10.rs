const INPUT: &str = include_str!("day10.txt");

const INPUT1: &str = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";

const INPUT2: &str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

fn main() {
    dbg!(part_1(INPUT1)); // 4
    dbg!(part_1(INPUT2)); // 8
                          // dbg!(part_2(INPUT1));

    // dbg!(part_1(INPUT));
    // dbg!(part_2(INPUT));
}

fn part_1(input: &str) -> usize {
    let map = parse_input(input);
    // dbg!(&map);

    // 1. Search start
    // 2. start loop
    // 3. find one connecting piece (which was not found before)
    // 4. change position to piece
    // 5. count up
    // 6. break if piece is start
    // 7. goto loop start
    // 8. loop end

    let mut positions = Vec::new();
    let start_pos = find_start(&map);
    positions.push(start_pos);

    // loop {
    //     todo!()
    // }

    positions.len() / 2
}

fn part_2(input: &str) -> i64 {
    todo!()
}

fn parse_input(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .map(str::trim)
        .map(|line| line.chars().map(Tile::from).collect())
        .collect()
}

#[derive(Debug)]
enum Tile {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'L' => Self::NorthEast,
            'J' => Self::NorthWest,
            '7' => Self::SouthWest,
            'F' => Self::SouthEast,
            '.' => Self::Ground,
            'S' => Self::Start,
            _ => unreachable!(),
        }
    }
}

fn find_start(map: &Vec<Vec<Tile>>) -> [usize; 2] {
    let start_pos = map
        .iter()
        .enumerate()
        .find_map(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .find(|(_col_idx, tile)| matches!(tile, Tile::Start))
                .map(|(col_idx, _tile)| [row_idx, col_idx])
        })
        .unwrap();
    dbg!(start_pos);
    start_pos
}
