fn main() {
    let input = include_str!("day04.txt");
    let part_1 = part_1(input);
    let part_2 = part_2(input);
    dbg!(part_1, part_2);
}

fn part_1(_input: &str) -> u32 {
    todo!()
}

fn part_2(_input: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        // Arrange
        let input = "";

        // Act
        let a = part_1(input);

        // Assert
        assert_eq!(a, 0);
    }

    #[test]
    fn test_2() {
        // Arrange
        let input = "";

        // Act
        let a = part_2(input);

        // Assert
        assert_eq!(a, 0);
    }
}
