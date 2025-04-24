use crate::{OrderingRule, is_correctly_ordered, parse_input};

pub fn result(input: &str) -> i32 {
    let input = parse_input(input);

    input
        .updates
        .iter()
        .filter(|update| !is_correctly_ordered(&input.ordering_rules, update))
        .map(|update| {
            let mut sorted = update.clone();

            sorted.sort_by(|a, b| {
                if input
                    .ordering_rules
                    .contains(&OrderingRule { x: *a, y: *b })
                {
                    return std::cmp::Ordering::Greater;
                }

                if input
                    .ordering_rules
                    .contains(&OrderingRule { x: *b, y: *a })
                {
                    return std::cmp::Ordering::Less;
                }

                std::cmp::Ordering::Equal
            });

            sorted
        })
        .fold(0, |acc, update: Vec<i32>| acc + update[update.len() / 2])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{INPUT, SAMPLE};

    #[test]
    fn test_sample() {
        let result = result(SAMPLE);

        assert_eq!(result, 123);
    }

    #[test]
    fn test_input() {
        let result = result(INPUT);

        assert_eq!(result, 5502);
    }
}
