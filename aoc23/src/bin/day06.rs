const INPUT: &str = include_str!("day06.txt");

fn main() {
    let input = "Time:      7  15   30
    Distance:  9  40  200";

    dbg!(part_1(input));
    // dbg!(part_2(input));

    dbg!(part_1(INPUT));
    // dbg!(part_2(INPUT));
}

fn part_1(input: &str) -> u32 {
    let a = input
        .lines()
        .map(str::trim)
        .map(parse_line)
        .collect::<Vec<_>>();

    let b = a[0]
        .iter()
        .copied()
        .zip(a[1].iter().copied())
        .collect::<Vec<_>>();
    // dbg!(b);

    let mut result = 1;
    for (time, record) in b {
        let mut win_counter = 0;
        for speed in 0..=time {
            let distance = (time - speed) * speed;
            if distance > record {
                win_counter += 1;
            }
        }
        result *= win_counter;
    }
    result
}

// fn part_2(input: &str) -> u32 {
//     todo!()
// }

fn parse_line(line: &str) -> Vec<u32> {
    line.split(' ')
        .skip(1)
        .map(str::trim)
        .filter_map(|b| b.parse::<u32>().ok())
        .collect::<Vec<_>>()
}
