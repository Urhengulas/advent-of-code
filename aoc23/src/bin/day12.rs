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
    // dbg!(&a);

    // 1. generate every possible combination -> replace ? with # and .
    // 2. filter if it fits condition
    // 3. sum

    let mut counter = 0;

    for (tokens, condition) in a {
        let combinations = generate_combinations(tokens);
        dbg!(combinations.len());
    }

    todo!()
}

// fn part_2(input: &str) -> usize {
//     todo!()
// }

#[derive(Clone, Debug, PartialEq)]
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

fn generate_combinations(tokens: Vec<Token>) -> Vec<Vec<Token>> {
    let unknown_count = tokens
        .iter()
        .filter(|token| **token == Token::Unknown)
        .count();
    let combination_count = 2_usize.pow(unknown_count as u32);

    let mut combinations = Vec::with_capacity(combination_count);
    combinations.push(tokens);

    while combinations[0].contains(&Token::Unknown) {
        // replace every combination with two combinations with ? replaced
        combinations = combinations
            .into_iter()
            .flat_map(|mut comb| {
                let idx = comb
                    .iter()
                    .position(|token| *token == Token::Unknown)
                    .unwrap();

                let mut comb1 = comb.clone();
                comb1[idx] = Token::Operational;

                comb[idx] = Token::Damaged;

                [comb, comb1]
            })
            .collect();
    }

    combinations
}
