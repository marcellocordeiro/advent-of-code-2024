use crate::{Pattern, parse_input};

pub fn result(input: &str) -> i32 {
    let patterns = parse_input(input);

    patterns
        .into_iter()
        .filter_map(|pattern| {
            if let Pattern::Mul { a, b } = pattern {
                Some((a, b))
            } else {
                None
            }
        })
        .fold(0, |acc, (a, b)| acc + (a * b))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{INPUT, SAMPLE_1 as SAMPLE};

    #[test]
    fn test_sample() {
        let result = result(SAMPLE);

        assert_eq!(result, 161);
    }

    #[test]
    fn test_input() {
        let result = result(INPUT);

        assert_eq!(result, 178886550);
    }
}
