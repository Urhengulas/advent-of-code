use std::{iter::Cycle, ops::RangeInclusive};

fn main() {
    dbg!(dirac_dice([6, 4]));
}

fn dirac_dice(input: [usize; 2]) -> usize {
    let mut die = Die::new();
    let mut players = [Player::new(input[0]), Player::new(input[1])];

    let mut round = 0;
    loop {
        // select current player
        let player = &mut players[round % 2];

        // roll the die
        let rolled = die.roll_n_times::<3>().into_iter().sum();

        // move the player and add points
        player.mv(rolled);

        round += 1;

        // exit if any player has won
        if player.score >= 1000 {
            break;
        }
    }

    let times_rolled = 3 * round;
    let loser_score = &players[round % 2].score;
    times_rolled * loser_score
}

#[derive(Debug)]
struct Player {
    pos: usize,
    score: usize,
}

impl Player {
    fn new(starting_pos: usize) -> Self {
        Self {
            pos: starting_pos,
            score: 0,
        }
    }

    fn mv(&mut self, rolled: usize) {
        self.pos = ((self.pos + rolled - 1) % 10) + 1;
        self.score += self.pos;
    }
}

struct Die(Cycle<RangeInclusive<usize>>);

impl Die {
    fn new() -> Self {
        Self((1..=100).cycle())
    }

    fn roll(&mut self) -> usize {
        self.0.next().unwrap()
    }

    fn roll_n_times<const N: usize>(&mut self) -> [usize; N] {
        (0..N)
            .map(|_| self.roll())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(dirac_dice([4, 8]), 739785);
    }
}
