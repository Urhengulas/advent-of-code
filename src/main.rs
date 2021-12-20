fn main() {
    let input = parse(include_str!("input.txt"));
    dbg!(hydrothermal_venture(&input));
}

fn parse(input: &str) -> Vec<(Coordinate, Coordinate)> {
    input
        .lines()
        .map(|line| {
            let (start, end) = line.split_once(" -> ").unwrap();

            let parse_coordinate = |s: &str| {
                let (y, x) = s.trim_matches(' ').split_once(',').expect(s);
                let (y, x) = (y.parse().expect(y), x.parse().expect(x));
                Coordinate { x, y }
            };

            (parse_coordinate(start), parse_coordinate(end))
        })
        .collect()
}

fn hydrothermal_venture(input: &[(Coordinate, Coordinate)]) -> usize {
    let mut grid = Grid::new();

    for (start, end) in input {
        grid.draw_line(start, end);
    }

    grid.count_overlaps()
}

#[derive(Debug)]
// Note: Needs to be `Box<[[usize; DIM]>`, because `[[usize; DIM]; DIM]` overflows the stack
struct Grid(Box<[[usize; DIM]]>);

const DIM: usize = 1000;

impl Grid {
    fn new() -> Self {
        Self(vec![[0; DIM]; DIM].into_boxed_slice())
    }

    fn draw_line(&mut self, start: &Coordinate, end: &Coordinate) {
        let calc_dimension = |s: usize, e| {
            let diff = e as isize - s as isize;
            let step = match diff {
                0 => 0,
                _ if diff < 0 => -1,
                _ => 1,
            };
            (diff.abs(), step)
        };

        let (x_diff, x_step) = calc_dimension(start.x, end.x);
        let (y_diff, y_step) = calc_dimension(start.y, end.y);
        let diff_max = (x_diff).max(y_diff);

        // increment all points in line
        for i in 0..=diff_max {
            let (x, y) = (
                (start.x as isize + i * x_step) as usize,
                (start.y as isize + i * y_step) as usize,
            );
            self.0[x][y] += 1;
        }
    }

    fn count_overlaps(self) -> usize {
        const OVERLAP_TRESHHOLD: usize = 2;
        self.0
            .into_iter()
            .flatten()
            .filter(|c| **c >= OVERLAP_TRESHHOLD)
            .count()
    }
}

#[derive(Debug)]
struct Coordinate {
    x: usize,
    y: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            hydrothermal_venture(&parse(
                "0,9 -> 5,9
                8,0 -> 0,8
                9,4 -> 3,4
                2,2 -> 2,1
                7,0 -> 7,4
                6,4 -> 2,0
                0,9 -> 2,9
                3,4 -> 1,4
                0,0 -> 8,8
                5,5 -> 8,2"
            )),
            12
        );
    }
}
