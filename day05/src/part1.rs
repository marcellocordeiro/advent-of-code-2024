use crate::{is_safe, parse_input};

pub fn result(input: &str) -> usize {
    let reports = parse_input(input);

    reports.into_iter().filter(|report| is_safe(report)).count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{INPUT, SAMPLE};

    #[test]
    fn test_sample() {
        let result = result(SAMPLE);

        assert_eq!(result, 2);
    }

    #[test]
    fn test_input() {
        let result = result(INPUT);

        assert_eq!(result, 549);
    }
}
