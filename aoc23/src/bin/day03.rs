const INPUT: &str = include_str!("day03.txt");

fn main() {
    let input = "467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598..";

    dbg!(part_1(input));
    // dbg!(part_2(input));

    // dbg!(part_1(INPUT));
    // dbg!(part_2(INPUT));
}

fn part_1(input: &str) -> u32 {
    let a = input
        .lines()
        .map(str::trim)
        .map(parse_line)
        // .inspect(|a| {
        //     dbg!(a);
        // })
        .collect::<Vec<_>>();
    dbg!(a);

    for line in a {
        let number_start = match line.iter().enumerate().find(|(b, c)|matches!(c, A::Number(_))) {
            Some((b, c)) => b,
            None => continue,
        };
        let number_end = match line.iter().enumerate().skip(number_start).find(|(b, c)|matches!(c, A::Number(_))) {
            Some((b, c)) => b,
            None => todo!(),
        }
        
        
        let next_idx = 
    }

    todo!()
}

fn parse_line(s: &str) -> Vec<A> {
    let mut result = Vec::with_capacity(s.len());
    for c in s.chars() {
        if c == '.' {
            result.push(A::Empty);
        } else if c.is_numeric() {
            let num = c.to_string().parse::<u32>().unwrap();
            result.push(A::Number(num));
        } else {
            result.push(A::Partnumber)
        }
    }
    result
}

#[derive(Debug)]
enum A {
    Number(u32),
    Empty,
    Partnumber,
}

// fn part_2(input: &str) -> u32 {
//     todo!()
// }
