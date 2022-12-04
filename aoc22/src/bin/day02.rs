#![feature(iter_array_chunks)]

use std::cmp::Ordering;

fn main() {
    let input = include_str!("day02.txt");
    let a = part_1(input);
    dbg!(a);
}

fn part_1(input: &str) -> u32 {
    input
        .chars()
        .filter(|a| ['A', 'B', 'C', 'X', 'Y', 'Z'].contains(&a))
        .map(Into::into)
        .array_chunks::<2>()
        .map(|[other_choice, my_choice]| {
            let shape_score = match my_choice {
                Choice::Rock => 1,
                Choice::Paper => 2,
                Choice::Scissors => 3,
            };
            let outcome_score = match is_winner(&[my_choice, other_choice]) {
                Ordering::Less => 0,
                Ordering::Equal => 3,
                Ordering::Greater => 6,
            };
            shape_score + outcome_score
        })
        .sum()
}

#[derive(Debug, PartialEq)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl From<char> for Choice {
    fn from(value: char) -> Self {
        match value {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            _ => unreachable!("malformed input"),
        }
    }
}

impl PartialOrd for Choice {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use Choice::*;
        use Ordering::*;
        Some(match (self, other) {
            (Rock, Rock) => Equal,
            (Paper, Paper) => Equal,
            (Scissors, Scissors) => Equal,

            (Rock, Scissors) => Greater,
            (Paper, Rock) => Greater,
            (Scissors, Paper) => Greater,

            (Paper, Scissors) => Less,
            (Scissors, Rock) => Less,
            (Rock, Paper) => Less,
        })
    }
}

fn is_winner(a: &[Choice; 2]) -> Ordering {
    PartialOrd::partial_cmp(&a[0], &a[1]).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::part_1;

    #[test]
    fn test_1() {
        // Arrange
        let input = "A Y
        B X
        C Z";

        // Act
        let a = part_1(input);

        // Assert
        assert_eq!(a, 15);
    }
}
