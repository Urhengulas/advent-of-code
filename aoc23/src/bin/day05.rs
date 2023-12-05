use std::ops::Range;

const INPUT: &str = include_str!("day05.txt");

fn main() {
    let input = "seeds: 79 14 55 13

    seed-to-soil map:
    50 98 2
    52 50 48
    
    soil-to-fertilizer map:
    0 15 37
    37 52 2
    39 0 15
    
    fertilizer-to-water map:
    49 53 8
    0 11 42
    42 0 7
    57 7 4
    
    water-to-light map:
    88 18 7
    18 25 70
    
    light-to-temperature map:
    45 77 23
    81 45 19
    68 64 13
    
    temperature-to-humidity map:
    0 69 1
    1 0 69
    
    humidity-to-location map:
    60 56 37
    56 93 4";

    dbg!(part_1(input));
    // dbg!(part_2(input));

    dbg!(part_1(INPUT));
    // dbg!(part_2(INPUT));
}

fn part_1(input: &str) -> u128 {
    let vecvec = str_to_vecvec(input);
    // dbg!(&vecvec);

    let (mut seeds, maps) = vecvec_to_maps(vecvec);
    // dbg!(&seeds, &maps);

    // dbg!(&seeds);
    for map in maps {
        for seed_idx in 0..seeds.len() {
            let seed = seeds[seed_idx];
            for (destination, source) in &map {
                if source.contains(&seed) {
                    seeds[seed_idx] = destination.start + (seed - source.start);
                }
            }
        }
        // dbg!(&seeds);
    }

    seeds.into_iter().min().unwrap()
}

// fn part_2(input: &str) -> u128 {
//     todo!()
// }

fn str_to_vecvec(s: &str) -> Vec<Vec<&str>> {
    let a = s.lines().map(str::trim).collect::<Vec<_>>();

    let mut b = vec![vec![]];
    let mut c = 0;
    for d in a {
        if d == "" {
            c += 1;
            b.push(Vec::new());
        } else {
            b[c].push(d);
        }
    }

    b
}

fn vecvec_to_maps(vecvec: Vec<Vec<&str>>) -> (Vec<u128>, Vec<Vec<(Range<u128>, Range<u128>)>>) {
    let mut seeds = Vec::new();
    let mut maps = Vec::new();
    for (idx, mut a) in vecvec.into_iter().enumerate() {
        if idx == 0 {
            let c = &a[0]["seeds: ".len()..];
            for d in c.split(" ") {
                let num = d.parse::<u128>().unwrap();
                seeds.push(num);
            }
        } else {
            a.remove(0);
            let c = a
                .into_iter()
                .map(|line| {
                    let d = line
                        .split(' ')
                        .map(|s| s.parse::<u128>().unwrap())
                        .collect::<Vec<_>>();
                    assert_eq!(d.len(), 3);

                    (d[0]..d[0] + d[2], d[1]..d[1] + d[2])
                })
                .collect::<Vec<_>>();
            maps.push(c);
        }
    }
    (seeds, maps)
}
