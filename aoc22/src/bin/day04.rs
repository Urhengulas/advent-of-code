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

fn part_2(_input: &str) -> u32 {
    todo!()
}

fn parse(line: &str) -> [RangeInclusive<u32>; 2] {
    let (a, b) = line.split_once(',').unwrap();

    let (c, d) = a.split_once('-').unwrap();
    let (e, f) = (c.parse().unwrap(), d.parse().unwrap());

    let (g, h) = b.split_once('-').unwrap();
    let (i, j) = (g.parse().unwrap(), h.parse().unwrap());

    [e..=f, i..=j]
}

fn fully_contains([a, b]: &[RangeInclusive<u32>; 2]) -> bool {
    let c = a.start();
    let d = a.end();
    let e = b.start();
    let f = b.end();

    // a contains b
    let g = c <= e && d >= f;

    // b contains a
    let h = e <= c && f >= d;

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
        assert_eq!(a, 0);
    }
}
