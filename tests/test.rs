// Templates

// pub fn parse(_input: &str) -> Vec<u32> {
//     vec![]
// }
//
// pub fn part1(_input: &[u32]) -> u32 {
//     123
// }
//
// pub fn part2(_input: &[u32]) -> u32 {
//     456
// }

// use aoc::year2025::day00::*;
//
// const EXAMPLE: &str = "";
//
// #[test]
// fn part1_test() {
//     let input = parse(EXAMPLE);
//     assert_eq!(part1(&input), 123);
// }
//
// #[test]
// fn part2_test() {
//     let input = parse(EXAMPLE);
//     assert_eq!(part2(&input), 456);
// }

macro_rules! test {
    ($year:tt $($day:tt),*) => {
        pub mod $year {$(pub mod $day;)*}
    }
}

test!(year2025
    day01
);
