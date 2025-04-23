use crate::{Pattern, parse_input};

pub fn result(input: &str) -> i32 {
    let patterns = parse_input(input);

    let mut enabled = true;
    let mut sum = 0;

    for pattern in patterns {
        match pattern {
            Pattern::Do => enabled = true,
            Pattern::Dont => enabled = false,
            Pattern::Mul { a, b } if enabled => sum += a * b,

            _ => {}
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{INPUT, SAMPLE_2 as SAMPLE};

    #[test]
    fn test_sample() {
        let result = result(SAMPLE);

        assert_eq!(result, 48);
    }

    #[test]
    fn test_input() {
        let result = result(INPUT);

        assert_eq!(result, 87163705);
    }
}
