use std::ops::Range;

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
    dbg!(part_2(input));

    dbg!(part_1(INPUT));
    dbg!(part_2(INPUT));
}

fn part_1(input: &str) -> u32 {
    let lines = input.lines().map(str::trim).collect::<Vec<_>>();

    find_part_numbers(&lines)
        .into_iter()
        .map(|pnum| pnum.num)
        .sum()
}

fn part_2(input: &str) -> u32 {
    let lines = input.lines().map(str::trim).collect::<Vec<_>>();
    let pnums = find_part_numbers(&lines);

    let mut sum = 0;

    for (line_idx, line) in lines.iter().enumerate() {
        let idx = match line.find('*') {
            Some(idx) => idx,
            None => continue,
        };

        // check previous, current and next line, except its first or last line
        let check_lines = get_check_lines(line_idx, lines.len());

        // check one char before and after, except it very beginning or end
        let range = idx.max(1)..=idx.min(line.len() - 2) + 2;

        // dbg!(&check_lines, &range);

        let mut adjacent_pnums = Vec::new();
        for pnum in &pnums {
            if check_lines.contains(&pnum.span.0) {
                // dbg!(&pnum);
                if range.contains(&pnum.span.1.start) || range.contains(&pnum.span.1.end) {
                    adjacent_pnums.push(pnum.num)
                }
            }
        }

        // dbg!(&adjacent_pnums);
        if adjacent_pnums.len() == 2 {
            sum += adjacent_pnums[0] * adjacent_pnums[1]
        }
    }

    sum
}

fn find_part_numbers(lines: &[&str]) -> Vec<PartNumber> {
    let mut pnums = Vec::new();

    'outer: for (line_idx, line) in lines.iter().enumerate() {
        let mut end_of_prev = 0;
        'inner: loop {
            // find beginning of num
            let number_start = match line[end_of_prev..].find(|c: char| c.is_ascii_digit()) {
                Some(idx) => end_of_prev + idx,
                None => continue 'outer,
            };
            // find end of num
            let number_end = match line[number_start..].find(|c: char| !c.is_ascii_digit()) {
                Some(idx) => number_start + idx,
                None => line.len(),
            };
            end_of_prev = number_end;

            // decide if it is part number or not

            // check previous, current and next line, except its first or last line
            let check_lines = get_check_lines(line_idx, lines.len());

            // check one char before and after, except it very beginning or end
            let (begin, end) = (number_start.max(1) - 1, number_end.min(line.len() - 2) + 1);

            // parse as number
            let num = line[number_start..number_end].parse::<u32>().unwrap();

            // go through lines and chars, add num of any char is symbol
            for idx in check_lines {
                let e = &lines[idx][begin..end];
                // dbg!(e);
                for f in e.chars() {
                    if f != '.' && !f.is_ascii_digit() {
                        pnums.push(PartNumber {
                            num,
                            span: (line_idx, number_start..number_end),
                        });
                        continue 'inner;
                    }
                }
            }
        }
    }
    pnums
}

fn get_check_lines(line_idx: usize, lines_len: usize) -> Vec<usize> {
    if line_idx == 0 {
        vec![line_idx, line_idx + 1]
    } else if line_idx == lines_len - 1 {
        vec![line_idx - 1, line_idx]
    } else {
        vec![line_idx - 1, line_idx, line_idx + 1]
    }
}

#[derive(Debug)]
struct PartNumber {
    num: u32,
    span: (usize, Range<usize>),
}
