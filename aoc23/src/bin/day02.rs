const INPUT: &str = include_str!("day02.txt");

fn main() {
    dbg!(part_1(INPUT));
    // dbg!(part_2(INPUT));
}

fn part_1(input: &str) -> u32 {
    let a = input
        .lines()
        .map(str::trim)
        .map(|line| line)
        .collect::<Vec<_>>();

    dbg!(a);

    todo!()
}

// fn part_2(input: &str) -> u32 {
//     todo!()
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test_1() {
        // Arrange
        let input = "";

        // Act
        let result = part_1(input);

        // Assert
        assert_eq!(result, 0);
    }
}
