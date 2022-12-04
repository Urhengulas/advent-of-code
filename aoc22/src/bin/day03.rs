use std::collections::HashSet;

fn main() {
    let input = include_str!("day03.txt");
    let part_1 = part_1(input);
    let part_2 = part_2(input);
    dbg!(part_1, part_2);
}

fn part_1(input: &str) -> u32 {
    let a = input.lines().map(str::trim).collect::<Vec<_>>();
    let mut sum = 0;
    for b in a {
        assert!(b.len() % 2 == 0, "{b}");
        let (c, d) = b.split_at(b.len() / 2);
        let (e, f) = (
            c.chars().collect::<HashSet<_>>(),
            d.chars().collect::<HashSet<_>>(),
        );
        let g = HashSet::intersection(&e, &f).collect::<Vec<_>>();
        assert!(g.len() == 1);
        let h = *g[0];
        let i = to_priority(h);
        sum += i;
    }
    sum
}

fn part_2(input: &str) -> () {
    ()
}

fn to_priority(a: char) -> u32 {
    let b = a as u32;
    match a.is_lowercase() {
        true => b - 96,
        false => b - 38,
    }
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
}
