#![feature(iter_array_chunks)]

use std::collections::HashSet;

fn main() {
    let input = include_str!("day03.txt");
    let part_1 = part_1(input);
    let part_2 = part_2(input);
    dbg!(part_1, part_2);
}

fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(str::trim)
        .inspect(|a| assert!(a.len() % 2 == 0))
        .map(|a| a.split_at(a.len() / 2))
        .map(|(a, b)| [to_char_set(a), to_char_set(b)])
        .map(|[a, b]| HashSet::intersection(&a, &b).cloned().collect::<Vec<_>>())
        .inspect(|a| assert!(a.len() == 1))
        .map(|a| to_priority(a[0]))
        .sum()
}

fn part_2(input: &str) -> u32 {
    input
        .lines()
        .map(str::trim)
        .array_chunks::<3>()
        .map(|[a, b, c]| [to_char_set(a), to_char_set(b), to_char_set(c)])
        .map(|a| intersection(a))
        .map(|a| to_priority(a))
        .sum()
}

fn to_char_set(a: &str) -> HashSet<char> {
    a.chars().collect()
}

fn to_priority(a: char) -> u32 {
    let b = a as u32;
    match a.is_lowercase() {
        true => b - 96,
        false => b - 38,
    }
}

fn intersection(a: [HashSet<char>; 3]) -> char {
    let b = a[0].intersection(&a[1]).cloned().collect::<HashSet<_>>();
    let c = a[2].intersection(&b).cloned().collect::<Vec<_>>();
    assert!(c.len() == 1);
    c[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        // Arrange
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw";

        // Act
        let a = part_1(input);

        // Assert
        assert_eq!(a, 157);
    }

    #[test]
    fn test_2() {
        // Arrange
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw";

        // Act
        let a = part_2(input);

        // Assert
        assert_eq!(a, 70);
    }
}
