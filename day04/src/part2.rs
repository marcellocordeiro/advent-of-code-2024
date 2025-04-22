use crate::{parse_input};

pub fn result(input: &str) -> i32 {
    let word_search = parse_input(input);

    let next_positions = [
        (1, 0),
        (0, 1),
        (1, 1),
    ];

    let mut count = 0;

    let i_size = word_search.len();
    let j_size = word_search[0].len();

    for next_position in next_positions {
        // Positive
        for i in (0..(i_size - 3)) {
            for j in (0..(j_size - 3)) {
                
            }
        }


    }

    count
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
