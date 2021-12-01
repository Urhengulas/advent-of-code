#![feature(array_windows)]

mod input;

use input::INPUT;

fn main() {
    dbg!(sonar_sweep(INPUT));
}

/// Count the number of times a depth measurement increases
fn sonar_sweep(measurements: &[u32]) -> usize {
    measurements
        .array_windows::<2>()
        .filter(|[a, b]| a < b)
        .count()
}

#[test]
fn test_1() {
    assert_eq!(
        sonar_sweep(&[199, 200, 208, 210, 200, 207, 240, 269, 260, 263]),
        7
    );
}
