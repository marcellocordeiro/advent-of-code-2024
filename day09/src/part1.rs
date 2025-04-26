use crate::{Block, checksum, parse_input};

pub fn result(input: &str) -> usize {
    let mut blocks = parse_input(input);

    for i in (1..blocks.len()).rev() {
        let block = blocks[i];

        if block == Block::Free {
            continue;
        }

        let first_free_block_position = blocks[..(i - 1)]
            .iter()
            .position(|b| matches!(b, Block::Free));

        if let Some(first_free_block_position) = first_free_block_position {
            blocks.swap(first_free_block_position, i);
        }
    }

    checksum(&blocks)
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
