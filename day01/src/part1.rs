use crate::parse_input;

pub fn result(input: &str) -> usize {
    let (mut left, mut right) = parse_input(input);

    left.sort_unstable();
    right.sort_unstable();

    left.into_iter()
        .zip(right)
        .fold(0, |acc, (left_num, right_num)| {
            let difference = left_num.abs_diff(right_num) as usize;
            acc + difference
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{INPUT, SAMPLE};

    #[test]
    fn test_sample() {
        let result = result(SAMPLE);

        assert_eq!(result, 11);
    }

    #[test]
    fn test_input() {
        let result = result(INPUT);

        assert_eq!(result, 2344935);
    }
}
