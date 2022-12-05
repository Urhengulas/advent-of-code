use std::ops::RangeInclusive;

fn main() {
    let input = include_str!("day04.txt");
    dbg!(part_1(input), part_2(input));
}

fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(str::trim)
        .map(parse)
        .filter(fully_contains)
        .count()
}

fn part_2(input: &str) -> usize {
    input
        .lines()
        .map(str::trim)
        .map(parse)
        .filter(overlap_at_all)
        .count()
}

fn parse(line: &str) -> [RangeInclusive<u32>; 2] {
    let (a, b) = line.split_once(',').unwrap();

    let c = |d: &str| {
        let (e, f) = d.split_once('-').unwrap();
        [e.parse().unwrap(), f.parse().unwrap()]
    };

    let [g, h] = c(a);
    let [i, j] = c(b);

    [g..=h, i..=j]
}

fn fully_contains([a, b]: &[RangeInclusive<u32>; 2]) -> bool {
    let [c, d] = [a.start(), a.end()];
    let [e, f] = [b.start(), b.end()];

    // a contains b
    let g = c <= e && d >= f;

    // b contains a
    let h = e <= c && f >= d;

    g || h
}

fn overlap_at_all([a, b]: &[RangeInclusive<u32>; 2]) -> bool {
    let [c, d] = [a.start(), a.end()];
    let [e, f] = [b.start(), b.end()];

    let g = d >= e && c <= f;
    let h = f >= c && e <= d;

    g || h
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8";

    #[test]
    fn test_1() {
        // Arrange

        // Act
        let a = part_1(INPUT);

        // Assert
        assert_eq!(a, 2);
    }

    #[test]
    fn test_2() {
        // Arrange

        // Act
        let a = part_2(INPUT);

        // Assert
        assert_eq!(a, 4);
    }
}
