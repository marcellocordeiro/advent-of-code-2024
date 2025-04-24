pub const INPUT: &str = include_str!("inputs/input.txt");
pub const SAMPLE: &str = include_str!("inputs/sample.txt");

pub mod part1;
pub mod part2;

#[derive(Debug)]
struct Equation {
    sum: i64,
    values: Vec<i64>,
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Mul,
    Concat,
}

impl Operation {
    fn apply(self, a: i64, b: i64) -> i64 {
        match self {
            Self::Add => a + b,
            Self::Mul => a * b,
            Self::Concat => format!("{a}{b}").parse().unwrap(), // Terribly inefficient
        }
    }
}

fn parse_input(input: &str) -> Vec<Equation> {
    input
        .lines()
        .map(|line| {
            line.split_once(':')
                .map(|(sum, values)| {
                    let values = values
                        .split_ascii_whitespace()
                        .map(|value| value.parse().unwrap())
                        .collect();

                    (sum.parse().unwrap(), values)
                })
                .unwrap()
        })
        .map(|(sum, values)| Equation { sum, values })
        .collect()
}

fn apply_operations(values: &[i64], ops: &[Operation]) -> i64 {
    assert!(values.len() == (ops.len() + 1));

    values[1..]
        .iter()
        .zip(ops.iter())
        .fold(values[0], |acc, (b, op)| op.apply(acc, *b))
}
