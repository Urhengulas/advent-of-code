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
        .map(intersection)
        .map(to_priority)
        .sum()
}

fn part_2(input: &str) -> u32 {
    input
        .lines()
        .map(str::trim)
        .array_chunks::<3>()
        .map(|[a, b, c]| [to_char_set(a), to_char_set(b), to_char_set(c)])
        .map(intersection)
        .map(to_priority)
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

fn intersection<const N: usize>(a: [HashSet<char>; N]) -> char {
    assert!(N > 0);
    let mut b = a.into_iter();

    // get intersection of all the sets
    let mut c = b.next().unwrap();
    for d in b {
        c = c.intersection(&d).cloned().collect::<HashSet<_>>();
    }

    // get the single element intersection
    let e = c.into_iter().collect::<Vec<_>>();
    assert!(e.len() == 1);
    e[0]
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
