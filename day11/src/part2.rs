use crate::{iterate, parse_input};

pub fn result(input: &str) -> usize {
    let mut stones = parse_input(input);

    //dbg!(&stones);
    for _ in 0..75 {
        iterate(&mut stones);
        //dbg!(&stones);
    }

    stones.len()
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{INPUT, SAMPLE};

    #[test]
    fn test_sample() {
        let result = result(SAMPLE);

        assert_eq!(result, 55312);
    }

    #[test]
    fn test_input() {
        let result = result(INPUT);

        assert_eq!(result, 189547);
    }
}
*/
