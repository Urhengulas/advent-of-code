const INPUT: &str = include_str!("day12.txt");

const INPUT1: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

fn main() {
    // assert_eq!(dbg!(part_1(INPUT1)), 21);
    // dbg!(part_1(INPUT));

    assert_eq!(dbg!(part_2(INPUT1)), 525152);
    // dbg!(part_2(INPUT));
}

fn part_1(input: &str) -> usize {
    let lines = parse_input(input);
    // dbg!(&a);

    // 1. generate every possible combination -> replace ? with # and .
    // 2. filter if it fits condition
    // 3. sum

    let mut counter = 0;

    for (tokens, condition) in lines {
        let combinations = generate_combinations(tokens);
        // dbg!(combinations.len());

        let count = combinations
            .into_iter()
            .filter(|comb| matches_condition(comb, &condition))
            // .inspect(|comb| {
            //     dbg!(comb);
            // })
            .count();
        counter += count;

        // dbg!(count);
        // println!();
    }

    counter
}

fn part_2(input: &str) -> usize {
    let lines = parse_input(input);
    let unfolded_lines = unfold(lines);

    let mut counter = 0;

    for (tokens, condition) in unfolded_lines {
        let combinations = generate_combinations(tokens);

        let count = combinations
            .into_iter()
            .filter(|comb| matches_condition(comb, &condition))
            .count();
        counter += count;
    }

    counter
}

#[derive(Clone, Debug, PartialEq)]
enum Token {
    Operational,
    Damaged,
    Unknown,
}

impl Token {
    fn is_unknown(&self) -> bool {
        *self == Self::Unknown
    }
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

type Line = (Vec<Token>, Vec<usize>);

fn parse_input(input: &str) -> Vec<Line> {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Line {
    let (a, b) = line.split_once(' ').unwrap();
    let c = a.chars().map(Into::into).collect();
    let d = b.split(',').map(|e| e.parse().unwrap()).collect();
    (c, d)
}

fn generate_combinations(tokens: Vec<Token>) -> Vec<Vec<Token>> {
    let unknown_count = tokens.iter().filter(|token| token.is_unknown()).count();
    let combination_count = 2_usize.pow(unknown_count as u32);

    let mut combinations = Vec::with_capacity(combination_count);
    combinations.push(tokens);

    while combinations[0].contains(&Token::Unknown) {
        // replace every combination with two combinations with ? replaced
        combinations = combinations
            .into_iter()
            .flat_map(|mut comb| {
                let idx = comb.iter().position(|token| token.is_unknown()).unwrap();

                let mut comb1 = comb.clone();
                comb1[idx] = Token::Operational;

                comb[idx] = Token::Damaged;

                [comb, comb1]
            })
            .collect();
    }

    combinations
}

fn matches_condition(mut combination: &[Token], condition: &[usize]) -> bool {
    let mut a = Vec::new();

    loop {
        // dbg!(combination);

        let Some(damaged_start) = combination
            .iter()
            .position(|token| *token == Token::Damaged)
        else {
            break;
        };
        combination = &combination[damaged_start..];

        let damaged_end = combination
            .iter()
            .position(|token| *token == Token::Operational)
            .unwrap_or(combination.len());
        combination = &combination[damaged_end..];

        a.push(damaged_end);
    }
    // dbg!(&a);

    a == condition
}

fn unfold(lines: Vec<Line>) -> Vec<Line> {
    lines.into_iter().map(unfold_line).collect()
}

fn unfold_line(line: Line) -> Line {
    todo!()
}
