fn main() {
    const INPUT: &str = include_str!("day01.txt");
    let a = part_1(INPUT);
    dbg!(a);
}

fn part_1(input: &str) -> u32 {
    let a = input.lines().map(str::trim).collect::<Vec<_>>();
    let mut c = vec![0];
    let mut c_idx = 0;
    for b in a {
        if b == "" {
            c_idx += 1;
            c.push(0);
            continue;
        } else {
            let d: u32 = b.parse().unwrap();
            c[c_idx] += d;
        }
    }
    c.into_iter().max().unwrap()
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
}
