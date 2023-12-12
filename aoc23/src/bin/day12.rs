const INPUT: &str = include_str!("day12.txt");

const INPUT1: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

fn main() {
    assert_eq!(dbg!(part_1(INPUT1)), 21);
    // dbg!(part_1(INPUT));

    // dbg!(part_2(INPUT1));
    // dbg!(part_2(INPUT));
}

fn part_1(input: &str) -> usize {
    let a = parse_input(input);
    dbg!(&a);

    todo!()
}

// fn part_2(input: &str) -> usize {
//     todo!()
// }

#[derive(Debug)]
enum Token {
    Operational,
    Damaged,
    Unknown,
}

impl From<char> for Token {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Operational,
            '#' => Self::Damaged,
            '?' => Self::Unknown,
            _ => unreachable!(),
        }
    }
}

fn parse_input(input: &str) -> Vec<(Vec<Token>, Vec<usize>)> {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> (Vec<Token>, Vec<usize>) {
    let (a, b) = line.split_once(' ').unwrap();
    let c = a.chars().map(Into::into).collect();
    let d = b.split(',').map(|e| e.parse().unwrap()).collect();
    (c, d)
}
