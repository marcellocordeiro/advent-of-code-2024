use crate::{is_safe, parse_input};

pub fn result(input: &str) -> usize {
    let reports = parse_input(input);

    reports
        .into_iter()
        .filter(|report| {
            // Test unmodified report
            if is_safe(report) {
                return true;
            }

            (0..report.len()).any(|index| {
                let modified = {
                    let mut modified = report.clone();
                    modified.remove(index);

                    modified
                };

                is_safe(&modified)
            })
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{INPUT, SAMPLE};

    #[test]
    fn test_sample() {
        let result = result(SAMPLE);

        assert_eq!(result, 4);
    }

    #[test]
    fn test_input() {
        let result = result(INPUT);

        assert_eq!(result, 589);
    }
}
