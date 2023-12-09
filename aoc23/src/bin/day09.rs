const INPUT: &str = include_str!("day09.txt");

const INPUT1: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

fn main() {
    dbg!(part_1(INPUT1));
    // dbg!(part_2(INPUT1));

    dbg!(part_1(INPUT));
    // dbg!(part_2(INPUT));
}

fn part_1(input: &str) -> i64 {
    let a = parse_input(input);
    // dbg!(&a);

    a.into_iter()
        .map(|b| {
            let c = history(b);
            extrapolate(c)
        })
        .sum()
}

// fn part_2(input: &str) -> usize {
//     todo!()
// }

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(str::trim)
        .map(|line| {
            line.split(' ')
                .map(|a| a.parse().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn history(a: Vec<i64>) -> Vec<Vec<i64>> {
    let mut b = vec![a];
    let mut idx = 0;
    loop {
        let c = b[idx].windows(2).map(|d| d[1] - d[0]).collect::<Vec<_>>();

        let all_zero = c.iter().all(|num| *num == 0);

        b.push(c);
        idx += 1;

        if all_zero {
            break;
        }
    }
    b
}

fn extrapolate(mut b: Vec<Vec<i64>>) -> i64 {
    for c in (1..b.len()).rev() {
        let d = *b[c].last().unwrap();
        let e = *b[c - 1].last().unwrap();
        b[c - 1].push(e + d);
    }
    // dbg!(&b);

    *b[0].last().unwrap()
}
