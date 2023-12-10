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
    // dbg!(part_1(INPUT1)); // 4
    // dbg!(part_1(INPUT2)); // 8
    dbg!(part_1(INPUT));

    // dbg!(part_2(INPUT1));
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

    // dbg!(tiles_around(start_pos, &map));

    'outer: loop {
        let pos = positions.last().unwrap();
        let tile = &map[pos[0]][pos[1]];
        // dbg!(pos, &tile);

        'inner: for around in tiles_around(*pos, &map) {
            let pos = around.pos();
            let tile_around = &map[pos[0]][pos[1]];
            // dbg!(tile_around);

            // end the search when encountering the start tile except, except we are
            // at the first tile after the start tile
            if matches!(tile_around, Tile::Start) {
                if positions.len() != 2 {
                    break 'outer;
                }
            }

            // skip tiles we already visited
            if positions.contains(&pos) {
                continue 'inner;
            }

            let is_connecting = 'block: {
                if tile.open_north() {
                    if matches!(around, Around::North(_)) {
                        if tile_around.open_south() {
                            break 'block true;
                        }
                    }
                }
                if tile.open_south() {
                    if matches!(around, Around::South(_)) {
                        if tile_around.open_north() {
                            break 'block true;
                        }
                    }
                }
                if tile.open_east() {
                    if matches!(around, Around::East(_)) {
                        if tile_around.open_west() {
                            break 'block true;
                        }
                    }
                }
                if tile.open_west() {
                    if matches!(around, Around::West(_)) {
                        if tile_around.open_east() {
                            break 'block true;
                        }
                    }
                }
                false
            };
            if is_connecting {
                positions.push(pos);
                continue 'outer;
            }
        }

        dbg!(&positions);
    }

    dbg!(&positions);
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
    /// |
    Vertical,
    /// -
    Horizontal,
    /// L
    NorthEast,
    /// J
    NorthWest,
    /// 7
    SouthWest,
    /// F
    SouthEast,
    /// .
    Ground,
    /// S
    Start,
}

impl Tile {
    fn open_north(&self) -> bool {
        match self {
            Tile::Vertical | Tile::NorthEast | Tile::NorthWest | Tile::Start => true,
            _ => false,
        }
    }

    fn open_south(&self) -> bool {
        match self {
            Tile::Vertical | Tile::SouthEast | Tile::SouthWest | Tile::Start => true,
            _ => false,
        }
    }

    fn open_west(&self) -> bool {
        match self {
            Tile::Horizontal | Tile::NorthWest | Tile::SouthWest | Tile::Start => true,
            _ => false,
        }
    }

    fn open_east(&self) -> bool {
        match self {
            Tile::Horizontal | Tile::NorthEast | Tile::SouthEast | Tile::Start => true,
            _ => false,
        }
    }
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
    // dbg!(start_pos);
    start_pos
}

#[derive(Debug)]
enum Around {
    North([usize; 2]),
    West([usize; 2]),
    East([usize; 2]),
    South([usize; 2]),
}

impl Around {
    fn pos(&self) -> [usize; 2] {
        match self {
            Around::North(a) | Around::West(a) | Around::East(a) | Around::South(a) => *a,
        }
    }
}

fn tiles_around(pos: [usize; 2], map: &Vec<Vec<Tile>>) -> Vec<Around> {
    let [row_idx, col_idx] = pos;
    let row_max = map.len() - 1;
    let col_max = map[0].len() - 1;

    let mut positions = Vec::with_capacity(4);

    if row_idx != 0 {
        positions.push(Around::North([row_idx - 1, col_idx]));
    }
    if col_idx != 0 {
        positions.push(Around::West([row_idx, col_idx - 1]));
    }
    if col_idx != col_max {
        positions.push(Around::East([row_idx, col_idx + 1]));
    }
    if row_idx != row_max {
        positions.push(Around::South([row_idx + 1, col_idx]));
    }

    positions
}
