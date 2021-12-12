fn main() {
    const INPUT: &str = include_str!("input.txt");
    let input = INPUT.lines().collect::<Vec<_>>();
    dbg!(binary_diagnostic(&input));
}

fn binary_diagnostic(input: &[&str]) -> u128 {
    // the most common bits
    let mut gamma = String::new();
    // the least common bits
    let mut epsilon = String::new();

    let input_len = input.len();
    let line_len = input[0].len();

    for i in 0..line_len {
        // count how often `1` occurred
        let mut sum = 0;
        for line in input {
            let digit = line.chars().nth(i).unwrap().to_digit(10).unwrap();
            sum += digit;
        }

        // which bit occurred more often
        let half = (input_len as u32) / 2;
        if sum > half {
            // `1` occurred more often than `0`
            gamma.push('1');
            epsilon.push('0');
        } else if sum < half {
            // `0` occurred more often than `1`
            gamma.push('0');
            epsilon.push('1');
        } else {
            // `0` and `1` occurred equally, which isn't handled by the task description
            unreachable!()
        }
    }

    let gamma = u128::from_str_radix(&gamma, 2).unwrap();
    let epsilon = u128::from_str_radix(&epsilon, 2).unwrap();

    gamma * epsilon
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            binary_diagnostic(&[
                "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
                "11001", "00010", "01010",
            ]),
            198
        );
    }
}
