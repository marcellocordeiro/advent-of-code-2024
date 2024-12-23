use crate::parse_input;

pub fn result(input: &str) -> i32 {
    let (left, right) = parse_input(input);

    left.into_iter().fold(0, |acc, left_num| {
        let right_count = right.iter().filter(|&&x| x == left_num).count() as i32;

        acc + (left_num * right_count)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{INPUT, SAMPLE};

    #[test]
    fn test_sample() {
        let result = result(SAMPLE);

        assert_eq!(result, 31);
    }

    #[test]
    fn test_input() {
        let result = result(INPUT);

        assert_eq!(result, 27647262);
    }
}
