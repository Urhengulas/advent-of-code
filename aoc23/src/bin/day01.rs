const INPUT: &str = include_str!("day01.txt");

fn main() {
    dbg!(part_1(INPUT));
    // dbg!(part_2(INPUT));
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

    dbg!(a);

    todo!()
}

// fn part_2(input: &str) -> u32 {
//     todo!()
// }
