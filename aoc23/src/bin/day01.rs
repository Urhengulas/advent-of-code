const INPUT: &str = include_str!("day01.txt");

fn main() {
    dbg!(part_1(INPUT));
    dbg!(part_2(INPUT));

    // dbg!(parse_line("j47three8sevenfivenfkd"));
}

fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(str::trim)
        .map(|line| {
            line.chars()
                .filter(|c: &char| c.is_numeric())
                .collect::<Vec<_>>()
        })
        .map(|numbers| {
            let a = numbers[0];
            let b = numbers[numbers.len() - 1];
            [a, b].into_iter().collect::<String>()
        })
        .map(|num_str| u32::from_str_radix(&num_str, 10).unwrap())
        .sum()
}

fn part_2(input: &str) -> u32 {
    input
        .lines()
        .map(str::trim)
        .map(parse_line)
        .map(|numbers| {
            let a = numbers[0].clone();
            let b = numbers[numbers.len() - 1].clone();
            let c = [a, b].into_iter().collect::<String>();
            c
        })
        .map(|num_str| u32::from_str_radix(&num_str, 10).unwrap())
        .sum()
}

fn parse_line(mut s: &str) -> Vec<String> {
    let mut numbers = Vec::new();

    'outer: loop {
        if s.starts_with(|c: char| c.is_digit(10)) {
            let digit_str = &s[0..1];
            s = &s[1..];
            numbers.push(digit_str.to_string());
            continue;
        }

        for (a, b) in DIGITS {
            if s.starts_with(b) {
                s = &s[b.len() - 1..];
                numbers.push(a.to_string());
                continue 'outer;
            }
        }

        if s.is_empty() {
            break;
        }

        s = &s[1..]
    }

    numbers
}

const DIGITS: [(u32, &str); 9] = [
    (1, "one"),
    (2, "two"),
    (3, "three"),
    (4, "four"),
    (5, "five"),
    (6, "six"),
    (7, "seven"),
    (8, "eight"),
    (9, "nine"),
];
