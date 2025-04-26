pub fn result(input: &str) -> usize {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{INPUT, SAMPLE};

    #[test]
    fn test_sample() {
        let result = result(SAMPLE);

        assert_eq!(result, 1928);
    }

    #[test]
    fn test_input() {
        let result = result(INPUT);

        assert_eq!(result, 6320029754031);
    }
}
