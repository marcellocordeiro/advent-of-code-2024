use itertools::{Itertools, repeat_n};

use crate::{Equation, Operation, apply_operations, parse_input};

pub fn result(input: &str) -> i64 {
    let equations = parse_input(input);

    equations
        .into_iter()
        .filter(verify_permutations)
        .map(|eq| eq.sum)
        .sum()
}

fn verify_permutations(equation: &Equation) -> bool {
    let sum = equation.sum;
    let values = &equation.values;

    repeat_n(
        [Operation::Add, Operation::Mul, Operation::Concat],
        values.len() - 1,
    )
    .multi_cartesian_product()
    .any(|ops| apply_operations(values, &ops) == sum)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{INPUT, SAMPLE};

    #[test]
    fn test_sample() {
        let result = result(SAMPLE);

        assert_eq!(result, 11387);
    }

    #[test]
    fn test_input() {
        let result = result(INPUT);

        assert_eq!(result, 661823605105500);
    }
}
