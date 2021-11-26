mod input;

use input::INPUT;

fn main() {
    let result = INPUT
        .lines()
        .map(|line| -> [u32; 3] {
            line.split("x")
                .map(|a| a.parse().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .map(|line| i_was_told_there_would_be_no_math(Present::new(line[0], line[1], line[2])))
        .sum::<u32>();
    dbg!(result);
}

struct Present {
    l: u32,
    w: u32,
    h: u32,
}

impl Present {
    fn new(l: u32, w: u32, h: u32) -> Self {
        Self { l, w, h }
    }

    fn smallest_side_area(&self) -> u32 {
        let mut sides = vec![self.l, self.w, self.h];

        // get the index of the highest value in `sides` ...
        let (max_idx, _) = sides.iter().enumerate().max_by_key(|a| a.1).unwrap();
        // ... and remove that highest value
        sides.remove(max_idx);

        assert!(sides.len() == 2);

        // multiply the remaining two sides
        sides[0] * sides[1]
    }

    fn total_area(&self) -> u32 {
        let Present { l, w, h } = self;
        2 * l * w + 2 * w * h + 2 * h * l
    }
}

fn i_was_told_there_would_be_no_math(present: Present) -> u32 {
    present.total_area() + present.smallest_side_area()
}

#[test]
fn test_1() {
    assert_eq!(i_was_told_there_would_be_no_math(Present::new(2, 3, 4)), 58);
}

#[test]
fn test_2() {
    assert_eq!(
        i_was_told_there_would_be_no_math(Present::new(1, 1, 10)),
        43
    );
}
