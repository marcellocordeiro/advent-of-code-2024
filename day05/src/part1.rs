use crate::{is_correctly_ordered, parse_input};

pub fn result(input: &str) -> i32 {
    let input = parse_input(input);

    input
        .updates
        .iter()
        .filter(|update| is_correctly_ordered(&input.ordering_rules, update))
        .fold(0, |acc, update: &Vec<i32>| acc + update[update.len() / 2])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{INPUT, SAMPLE};

    #[test]
    fn test_sample() {
        let result = result(SAMPLE);

        assert_eq!(result, 143);
    }

    #[test]
    fn test_input() {
        let result = result(INPUT);

        assert_eq!(result, 5747);
    }
}
