pub const INPUT: &str = include_str!("inputs/input.txt");
pub const SAMPLE: &str = include_str!("inputs/sample.txt");

pub mod part1;
pub mod part2;

pub fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    input.lines().fold((vec![], vec![]), |mut acc, line| {
        let (left, right) = line
            .split_once(' ')
            .map(|(left, right)| {
                (
                    left.trim().parse::<i32>().unwrap(),
                    right.trim().parse::<i32>().unwrap(),
                )
            })
            .unwrap();

        acc.0.push(left);
        acc.1.push(right);

        acc
    })
}
