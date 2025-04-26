use itertools::Itertools;

use crate::{Block, checksum, parse_input};

pub fn result(input: &str) -> usize {
    let mut blocks = parse_input(input);

    compact(&mut blocks);

    checksum(&blocks)
}

fn compact(blocks: &mut [Block]) {
    let ids = existing_ids(blocks);

    for current_id in ids.into_iter().rev() {
        let start = blocks
            .iter()
            .position(|block| *block == Block::FilePart(current_id))
            .unwrap();

        let end = (blocks.len() - 1)
            - blocks
                .iter()
                .rev()
                .position(|block| *block == Block::FilePart(current_id))
                .unwrap();

        let (search_slice, file_slice) = blocks[..=end].split_at_mut(start);

        let free_slice = find_free_slice(search_slice, file_slice.len());

        if let Some(free_slice) = free_slice {
            free_slice.swap_with_slice(file_slice);
        }
    }
}

fn existing_ids(blocks: &[Block]) -> Vec<usize> {
    blocks
        .iter()
        .filter_map(|block| match *block {
            Block::FilePart(id) => Some(id),
            Block::Free => None,
        })
        .dedup()
        .collect()
}

fn find_free_slice(slice: &mut [Block], size: usize) -> Option<&mut [Block]> {
    if size > slice.len() {
        return None;
    }

    let window_start = (0..=(slice.len() - size)).find(|window_start| {
        slice[*window_start..(*window_start + size)]
            .iter()
            .all(|block| *block == Block::Free)
    });

    window_start.map(|start| &mut slice[start..(start + size)])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{INPUT, SAMPLE};

    #[test]
    fn test_sample() {
        let result = result(SAMPLE);

        assert_eq!(result, 2858);
    }

    #[test]
    fn test_input() {
        let result = result(INPUT);

        assert_eq!(result, 6347435485773);
    }
}
