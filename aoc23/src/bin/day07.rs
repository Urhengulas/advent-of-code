use std::collections::HashMap;

const INPUT: &str = include_str!("day07.txt");

fn main() {
    let input = "32T3K 765
    T55J5 684
    KK677 28
    KTJJT 220
    QQQJA 483";

    dbg!(part_1(input));
    dbg!(part_1(input));

    dbg!(part_1(INPUT));
    dbg!(part_2(INPUT));
}

fn part_1(input: &str) -> usize {
    part_x(input, "11")
}

fn part_2(input: &str) -> usize {
    part_x(input, "0")
}

fn part_x(input: &str, j_str: &str) -> usize {
    let a = input
        .lines()
        .map(str::trim)
        .map(|line| parse_line(line, j_str))
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

    // // print ranked hands (for debugging)
    // for (idx, (hand, hand_type)) in c.iter().map(|a| (a.0, a.3)).enumerate() {
    //     println!("{idx:#03}: {hand_type} | {hand:?}");
    // }

    c.into_iter()
        .enumerate()
        .map(|(idx, (_, bid, _, _))| (idx + 1) * bid)
        .sum()
}

fn parse_line(line: &str, j_str: &str) -> ([u32; 5], usize) {
    let (cards, bid) = line.split_once(' ').unwrap();
    let bid = bid.parse().unwrap();

    let cards = cards
        .chars()
        .map(|c| match c {
            'T' => "10".to_string(),
            'J' => j_str.to_string(),
            'Q' => "12".to_string(),
            'K' => "13".to_string(),
            'A' => "14".to_string(),
            _ => c.to_string(),
        })
        .map(|s| s.parse().expect(&format!("{s:?}")))
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    (cards, bid)
}

fn count_cards(a: Vec<([u32; 5], usize)>) -> Vec<([u32; 5], usize, HashMap<u32, u32>)> {
    map(a, |(hand, bid)| {
        let mut occurences = HashMap::new();
        for card in hand {
            if occurences.get(&card).is_none() {
                let count = hand.iter().filter(|c| **c == card).count() as u32;
                occurences.insert(card, count);
            }
        }
        (hand, bid, occurences)
    })
}

fn rank_cards(
    b: Vec<([u32; 5], usize, HashMap<u32, u32>)>,
) -> Vec<([u32; 5], usize, HashMap<u32, u32>, u32)> {
    map(b, |(hand, bid, mut occurences)| {
        // dbg!(&occurences);
        let joker_count = occurences.remove(&0).unwrap_or(0);

        if joker_count == 5 {
            return (hand, bid, occurences, 6);
        }

        let most = *occurences.iter().max_by_key(|a| a.1).unwrap().0;
        occurences
            .entry(most)
            .and_modify(|count| *count += joker_count);
        // dbg!(&occurences);

        let max = *occurences.values().max().unwrap();

        let hand_type: u32 = match max {
            5 => 6,                                                              // five of a kind
            4 => 5,                                                              // four of a kind
            3 if occurences.values().any(|key| *key == 2) => 4,                  // full house
            3 => 3,                                                              // three of a kind
            2 if occurences.values().filter(|key| **key == 2).count() == 2 => 2, // two pair
            2 => 1,                                                              // pair
            _ => 0,                                                              // high card
        };
        // dbg!(hand_type);

        (hand, bid, occurences, hand_type)
    })
}

fn map<T, U, V>(data: Vec<T>, f: U) -> Vec<V>
where
    U: Fn(T) -> V,
{
    data.into_iter().map(f).collect()
}
