use std::iter;

const INPUT: &str = include_str!("day12.txt");

const INPUT1: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

fn main() {
    assert_eq!(dbg!(part_1(INPUT1)), 21);
    dbg!(part_1(INPUT));

    // assert_eq!(dbg!(part_2(INPUT1)), 525152);
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
    // dbg!(&unfolded_lines);
    println!("unfolded lines");

    let len = unfolded_lines.len();
    unfolded_lines
        .into_iter()
        .enumerate()
        .map(|(idx, (tokens, condition))| {
            println!("{}/{len}", idx + 1);

            let combinations = generate_combinations(tokens);

            let count = combinations
                .into_iter()
                .filter(|comb| matches_condition(comb, &condition))
                .count();

            count
        })
        .sum()
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

struct TokenCombinations {
    base_tokens: Vec<Token>,
    current_combination: usize,
    total_combinations: usize,
}

impl TokenCombinations {
    fn new(tokens: Vec<Token>) -> Self {
        let unknown_count = tokens.iter().filter(|token| token.is_unknown()).count();
        let total_combinations = 2_usize.pow(unknown_count as u32);

        TokenCombinations {
            base_tokens: tokens,
            current_combination: 0,
            total_combinations,
        }
    }
}

impl Iterator for TokenCombinations {
    type Item = Vec<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_combination >= self.total_combinations {
            return None;
        }

        let mut result = self.base_tokens.clone();
        let mut mask = self.current_combination;

        for token in result.iter_mut() {
            if token.is_unknown() {
                *token = if mask % 2 == 0 {
                    Token::Operational
                } else {
                    Token::Damaged
                };
                mask /= 2;
            }
        }

        self.current_combination += 1;
        Some(result)
    }
}

fn generate_combinations(tokens: Vec<Token>) -> impl Iterator<Item = Vec<Token>> {
    TokenCombinations::new(tokens)
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
    let (mut a, mut b) = line;
    a.reserve_exact(4 * a.len() + 4);
    b.reserve_exact(4 * a.len());

    let mut a = iter::repeat(a)
        .take(5)
        .flat_map(|e| [e, vec![Token::Unknown]])
        .flatten()
        .collect::<Vec<_>>();
    a.pop();

    let b = iter::repeat(b).take(5).flatten().collect();

    // dbg!(&a, &b);
    (a, b)
}
