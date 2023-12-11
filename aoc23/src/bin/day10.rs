use std::fmt::Debug;

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

const INPUT3: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

const INPUT4: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

fn main() {
    dbg!(part_1(INPUT1)); // 4
    dbg!(part_1(INPUT2)); // 8
    dbg!(part_1(INPUT));

    dbg!(part_2(INPUT3)); // 4
    dbg!(part_2(INPUT4)); // 10
    dbg!(part_2(INPUT));
}

fn part_1(input: &str) -> usize {
    let map = parse_input(input);
    // dbg!(&map);

    let positions = search(&map);
    // dbg!(&positions);

    positions.len() / 2
}

fn part_2(input: &str) -> u32 {
    let map = parse_input(input);
    // dbg!(&map);
    println!("parsed");

    let positions = search(&map);
    // dbg!(&positions);
    println!("searched");

    enclosed_tiles(map, &positions)
}

fn parse_input(input: &str) -> Map {
    input
        .lines()
        .map(str::trim)
        .map(|line| line.chars().map(Tile::from).collect())
        .collect()
}

type Map = Vec<Vec<Tile>>;

#[derive(Clone)]
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

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Vertical => write!(f, "|"),
            Self::Horizontal => write!(f, "-"),
            Self::NorthEast => write!(f, "L"),
            Self::NorthWest => write!(f, "J"),
            Self::SouthWest => write!(f, "7"),
            Self::SouthEast => write!(f, "F"),
            Self::Ground => write!(f, "."),
            Self::Start => write!(f, "S"),
        }
    }
}

type Pos = [usize; 2];

fn find_start(map: &Map) -> Pos {
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
    North(Pos),
    West(Pos),
    East(Pos),
    South(Pos),
}

impl Around {
    fn pos(&self) -> Pos {
        match self {
            Around::North(a) | Around::West(a) | Around::East(a) | Around::South(a) => *a,
        }
    }
}

fn tiles_around(pos: Pos, map: &Map) -> Vec<Around> {
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

// 1. Search start
// 2. start loop
// 3. find one connecting piece (which was not found before)
// 4. change position to piece
// 5. count up
// 6. break if piece is start
// 7. goto loop start
// 8. loop end
fn search(map: &Map) -> Vec<Pos> {
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

            // end the search when encountering the start tile, except we are at the
            // first tile after the start tile
            if matches!(tile_around, Tile::Start) && is_connecting && positions.len() != 2 {
                break 'outer;
            }

            // skip tiles we already visited
            if positions.contains(&pos) {
                continue 'inner;
            }

            if is_connecting {
                positions.push(pos);
                continue 'outer;
            }
        }
    }

    // dbg!(&positions);
    positions
}

fn enclosed_tiles(mut map: Map, positions: &[Pos]) -> u32 {
    let mut counter = 0;
    let mut inside = false;
    let mut last_curve = None;

    // set non-loop tile to ground
    for (row_idx, row) in map.iter_mut().enumerate() {
        for (col_idx, tile) in row.iter_mut().enumerate() {
            if !positions.contains(&[row_idx, col_idx]) {
                *tile = Tile::Ground;
            }
        }
    }

    for (row_idx, row) in map.iter().enumerate() {
        for (col_idx, tile) in row.iter().enumerate() {
            check_tile(
                positions,
                row_idx,
                col_idx,
                tile,
                &mut inside,
                &mut counter,
                &mut last_curve,
            )
        }
    }

    counter
}

fn check_tile(
    positions: &[Pos],
    row_idx: usize,
    col_idx: usize,
    tile: &Tile,
    inside: &mut bool,
    counter: &mut u32,
    last_curve: &mut Option<Tile>,
) {
    match tile {
        Tile::Vertical => *inside = !*inside,
        Tile::Start => check_tile(
            positions,
            row_idx,
            col_idx,
            &Tile::Vertical, // HACK: I know my "S" is a "|"
            inside,
            counter,
            last_curve,
        ),

        Tile::Ground if *inside => *counter += 1,
        Tile::Ground => { /* not inside */ }

        Tile::Horizontal => (),
        curve @ (Tile::NorthEast | Tile::NorthWest | Tile::SouthWest | Tile::SouthEast) => {
            match (curve, &last_curve) {
                (_, None) => *last_curve = Some(curve.clone()),
                (Tile::NorthWest, Some(Tile::NorthEast)) => *last_curve = None,
                (Tile::SouthWest, Some(Tile::NorthEast)) => {
                    *inside = !*inside;
                    *last_curve = None;
                }
                (Tile::SouthWest, Some(Tile::SouthEast)) => *last_curve = None,
                (Tile::NorthWest, Some(Tile::SouthEast)) => {
                    *inside = !*inside;
                    *last_curve = None;
                }
                _ => {
                    dbg!(row_idx, col_idx);
                    unreachable!()
                }
            }
        }
    }
}
