use std::collections::HashMap;

const INPUT: &str = include_str!("day07.txt");

fn main() {
    let input = "32T3K 765
    T55J5 684
    KK677 28
    KTJJT 220
    QQQJA 483";

    dbg!(part_1(input));
    // dbg!(part_2(input));

    dbg!(part_1(INPUT));
    // dbg!(part_2(INPUT));
}

fn part_1(input: &str) -> usize {
    let a = input
        .lines()
        .map(str::trim)
        .map(parse_line)
        .collect::<Vec<_>>();
    // dbg!(&a);

    let b = count_cards(a);
    let mut c = rank_cards(b);

    // sort by hand
    c.sort_by_key(|d| d.0);
    // dbg!(&c);

    // sort by hand_type
    c.sort_by_key(|d| d.3);
    // dbg!(&c);

    c.into_iter()
        .enumerate()
        .map(|(idx, (_, bid, _, _))| (idx + 1) * bid)
        .sum()
}

// fn part_2(input: &str) -> u64 {
//     todo!()
// }

fn parse_line(line: &str) -> ([u32; 5], usize) {
    let (cards, bid) = line.split_once(' ').unwrap();
    let bid = bid.parse::<usize>().unwrap();

    let cards = cards
        .chars()
        .map(|c| match c {
            'T' => "10".to_string(),
            'J' => "11".to_string(),
            'Q' => "12".to_string(),
            'K' => "13".to_string(),
            'A' => "14".to_string(),
            _ => c.to_string(),
        })
        .map(|s| s.parse::<u32>().expect(&format!("{s:?}")))
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    (cards, bid)
}

fn count_cards(a: Vec<([u32; 5], usize)>) -> Vec<([u32; 5], usize, HashMap<u32, u32>)> {
    let mut b = Vec::new();
    for (hand, bid) in a {
        let mut occurences = HashMap::new();
        for card in hand {
            if occurences.get(&card).is_none() {
                let count = hand.iter().filter(|c| **c == card).count() as u32;
                occurences.insert(card, count);
            }
        }
        b.push((hand, bid, occurences))
    }
    // dbg!(&b);
    b
}

fn rank_cards(
    b: Vec<([u32; 5], usize, HashMap<u32, u32>)>,
) -> Vec<([u32; 5], usize, HashMap<u32, u32>, u32)> {
    let mut c = Vec::new();
    for (hand, bid, occurences) in b {
        let hand_type: u32 = if occurences.values().any(|key| *key == 5) {
            // five of a kind
            6
        } else if occurences.values().any(|key| *key == 4) {
            // five of a kind
            5
        } else if occurences.values().any(|key| *key == 3)
            && occurences.values().any(|key| *key == 2)
        {
            // full house
            4
        } else if occurences.values().any(|key| *key == 3) {
            // three of a kind
            3
        } else if occurences.values().filter(|key| **key == 2).count() == 2 {
            // two pair
            2
        } else if occurences.values().any(|key| *key == 2) {
            // pair
            1
        } else {
            // high card
            0
        };
        // dbg!(hand_type);

        c.push((hand, bid, occurences, hand_type));
    }
    // dbg!(&c);
    c
}
