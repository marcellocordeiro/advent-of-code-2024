pub const INPUT: &str = include_str!("inputs/input.txt");
pub const SAMPLE: &str = include_str!("inputs/sample.txt");

pub mod part1;
pub mod part2;

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|str| str.parse().unwrap())
                .collect()
        })
        .collect()
}

fn is_safe(report: &[i32]) -> bool {
    let mut report_is_increasing = None;

    report.windows(2).all(|win| {
        let left = win[0];
        let right = win[1];

        let is_increasing = right > left;

        if report_is_increasing.is_none() {
            report_is_increasing = Some(is_increasing);
        }

        if Some(is_increasing) == report_is_increasing {
            let diff = right.abs_diff(left);

            (1..=3).contains(&diff)
        } else {
            false
        }
    })
}
