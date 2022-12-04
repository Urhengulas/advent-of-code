fn main() {
    const INPUT: &str = include_str!("day01.txt");

    let a = part_1(INPUT);
    dbg!(a);

    let b = part_2(INPUT);
    dbg!(b);
}

fn part_1(input: &str) -> u32 {
    let c = parse(input);
    c.into_iter().max().unwrap()
}

fn part_2(input: &str) -> u32 {
    let mut a = parse(input);
    a.sort_unstable();
    let b = &a[a.len() - 3..a.len()];
    b.iter().sum()
}

fn parse(input: &str) -> Vec<u32> {
    let a = input.lines().map(str::trim).collect::<Vec<_>>();
    let mut c = vec![0];
    let mut c_idx = 0;
    for b in a {
        if b.is_empty() {
            c_idx += 1;
            c.push(0);
            continue;
        } else {
            let d: u32 = b.parse().unwrap();
            c[c_idx] += d;
        }
    }
    c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        // Arrange
        const INPUT: &str = "1000
        2000
        3000

        4000

        5000
        6000

        7000
        8000
        9000

        10000";

        // Act
        let a = part_1(INPUT);

        // Assert
        assert_eq!(a, 24000);
    }

    #[test]
    fn test_2() {
        // Arrange
        const INPUT: &str = "1000
        2000
        3000

        4000

        5000
        6000

        7000
        8000
        9000

        10000";

        // Act
        let a = part_2(INPUT);

        // Assert
        assert_eq!(a, 45000);
    }
}
