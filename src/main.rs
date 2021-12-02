mod input;

use input::INPUT;

fn main() {
    let directions = INPUT.iter().map(|&c| c.into()).collect::<Vec<_>>();
    dbg!(dive(&directions));
}

enum Direction {
    Up(u128),
    Down(u128),
    Forward(u128),
}

impl From<&str> for Direction {
    fn from(v: &str) -> Self {
        if let Some(s) = v.strip_prefix("up ") {
            Self::Up(s.parse().unwrap())
        } else if let Some(s) = v.strip_prefix("down ") {
            Self::Down(s.parse().unwrap())
        } else if let Some(s) = v.strip_prefix("forward ") {
            Self::Forward(s.parse().unwrap())
        } else {
            unreachable!()
        }
    }
}

struct Position {
    /// Horizontal position
    x: u128,
    /// Depth
    y: u128,
    /// Aim
    z: u128,
}

impl Position {
    fn new() -> Self {
        Self { x: 0, y: 0, z: 0 }
    }

    fn up(&mut self, v: u128) {
        self.z -= v;
    }

    fn down(&mut self, v: u128) {
        self.z += v;
    }

    fn forward(&mut self, v: u128) {
        self.x += v;
        self.y += self.z * v;
    }
}

fn dive(directions: &[Direction]) -> u128 {
    let mut pos = Position::new();

    for dir in directions {
        match dir {
            Direction::Up(v) => pos.up(*v),
            Direction::Down(v) => pos.down(*v),
            Direction::Forward(v) => pos.forward(*v),
        };
    }

    pos.x * pos.y
}

#[cfg(test)]
mod tests {
    use super::*;

    use Direction::*;

    #[test]
    fn test_1() {
        assert_eq!(
            dive(&[Forward(5), Down(5), Forward(8), Up(3), Down(8), Forward(2)]),
            900
        );
    }
}
