mod input;

use std::collections::HashSet;

use input::INPUT;

fn main() {
    let directions = INPUT.chars().map(|c| c.into()).collect::<Vec<_>>();
    dbg!(perfectly_spherical_houses_in_a_vacuum(&directions));
}

enum Direction {
    North,
    East,
    South,
    West,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '^' => Self::North,
            '>' => Self::East,
            'v' => Self::South,
            '<' => Self::West,
            _ => unreachable!(),
        }
    }
}

/// Count the number of times a depth measurement increases
fn perfectly_spherical_houses_in_a_vacuum(directions: &[Direction]) -> usize {
    let mut visited = HashSet::new();

    let mut x = 0;
    let mut y = 0;

    visited.insert([x, y]);

    for dir in directions {
        match dir {
            Direction::North => y += 1,
            Direction::East => x += 1,
            Direction::South => y -= 1,
            Direction::West => x -= 1,
        }

        visited.insert([x, y]);
    }

    visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    use Direction::*;

    #[test]
    fn test_1() {
        assert_eq!(perfectly_spherical_houses_in_a_vacuum(&[East]), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(
            perfectly_spherical_houses_in_a_vacuum(&[North, East, South, West]),
            4
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            perfectly_spherical_houses_in_a_vacuum(&[
                North, South, North, South, North, South, North, South
            ]),
            2
        );
    }
}
