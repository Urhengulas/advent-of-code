#![feature(iter_array_chunks)]

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
            let outcome_score = match is_winner(&my_choice, &other_choice) {
                Outcome::Loose => 0,
                Outcome::Draw => 3,
                Outcome::Win => 6,
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

enum Outcome {
    Win,
    Loose,
    Draw,
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

fn is_winner(my_choice: &Choice, other_choice: &Choice) -> Outcome {
    use {Choice::*, Outcome::*};

    match (my_choice, other_choice) {
        (Rock, Rock) => Draw,
        (Paper, Paper) => Draw,
        (Scissors, Scissors) => Draw,

        (Rock, Scissors) => Win,
        (Paper, Rock) => Win,
        (Scissors, Paper) => Win,

        (Paper, Scissors) => Loose,
        (Scissors, Rock) => Loose,
        (Rock, Paper) => Loose,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
