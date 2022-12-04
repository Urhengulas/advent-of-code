#![feature(iter_array_chunks)]

fn main() {
    let input = include_str!("day02.txt");
    let part_1 = part_1(input);
    let part_2 = part_2(input);
    dbg!(part_1, part_2);
}

fn part_1(input: &str) -> u32 {
    input
        .chars()
        .filter(|a| ['A', 'B', 'C', 'X', 'Y', 'Z'].contains(a))
        .map(Into::into)
        .array_chunks::<2>()
        .map(|[other_choice, my_choice]| calculate_score(&my_choice, &other_choice))
        .sum()
}

fn part_2(input: &str) -> u32 {
    input
        .chars()
        .filter(|a| ['A', 'B', 'C', 'X', 'Y', 'Z'].contains(a))
        .array_chunks::<2>()
        .map(|[a, b]| (a.into(), b.into()))
        .map(|(other_choice, outcome)| {
            let my_choice = choose_choice(&other_choice, &outcome);
            calculate_score(&my_choice, &other_choice)
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
            _ => unreachable!(),
        }
    }
}

impl From<char> for Outcome {
    fn from(value: char) -> Self {
        match value {
            'X' => Self::Loose,
            'Y' => Self::Draw,
            'Z' => Self::Win,
            _ => unreachable!(),
        }
    }
}

fn calculate_score(my_choice: &Choice, other_choice: &Choice) -> u32 {
    let shape_score = match my_choice {
        Choice::Rock => 1,
        Choice::Paper => 2,
        Choice::Scissors => 3,
    };
    let outcome_score = match is_winner(my_choice, other_choice) {
        Outcome::Loose => 0,
        Outcome::Draw => 3,
        Outcome::Win => 6,
    };
    shape_score + outcome_score
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

fn choose_choice(other_choice: &Choice, outcome: &Outcome) -> Choice {
    use {Choice::*, Outcome::*};
    match (other_choice, outcome) {
        (Rock, Win) => Paper,
        (Rock, Loose) => Scissors,
        (Rock, Draw) => Rock,
        (Paper, Win) => Scissors,
        (Paper, Loose) => Rock,
        (Paper, Draw) => Paper,
        (Scissors, Win) => Rock,
        (Scissors, Loose) => Paper,
        (Scissors, Draw) => Scissors,
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

    #[test]
    fn test_2() {
        // Arrange
        let input = "A Y
        B X
        C Z";

        // Act
        let a = part_2(input);

        // Assert
        assert_eq!(a, 12);
    }
}
