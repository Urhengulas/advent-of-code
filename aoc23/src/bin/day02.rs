const INPUT: &str = include_str!("day02.txt");

fn main() {
    // let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    // Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    // Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    // Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    // Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    // dbg!(part_2(input));
    dbg!(part_1(INPUT));
    dbg!(part_2(INPUT));
}

fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(str::trim)
        .map(parse_line)
        .enumerate()
        .filter(|(_, color)| color.r <= 12 && color.g <= 13 && color.b <= 14)
        // .inspect(|a| {
        //     dbg!(a);
        // })
        .map(|(idx, _)| idx + 1)
        .sum()
}

fn parse_line(s: &str) -> Color {
    // skip "Game "
    let s = &s[5..];

    let (_id, s) = s.split_once(": ").unwrap();

    let mut result = Color { r: 0, g: 0, b: 0 };

    for a in s.split("; ") {
        for b in a.split(", ") {
            let (num, color) = b.split_once(' ').unwrap();
            let num = num.parse::<u32>().unwrap();
            match color {
                "red" => result.r = num.max(result.r),
                "green" => result.g = num.max(result.g),
                "blue" => result.b = num.max(result.b),
                _ => unreachable!(),
            }
        }
    }

    result
}

#[derive(Debug)]
struct Color {
    r: u32,
    g: u32,
    b: u32,
}

fn part_2(input: &str) -> u32 {
    input
        .lines()
        .map(str::trim)
        .map(parse_line)
        .map(|color| dbg!(color.r * color.g * color.b))
        .sum()
}
