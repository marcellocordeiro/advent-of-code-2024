pub fn count_digits(num: usize) -> usize {
    if num == 0 {
        return 1;
    }

    (num.ilog10() + 1) as usize
}

pub fn split_digits_in_half(num: usize) -> (usize, usize) {
    let digits = count_digits(num) as u32;
    let factor = 10_usize.pow(digits / 2);

    let left = num / factor;
    let right = num % factor;

    (left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_digits() {
        assert!(count_digits(0) == 1);
        assert!(count_digits(1) == 1);
        assert!(count_digits(2) == 1);
        assert!(count_digits(01) == 1);
        assert!(count_digits(10) == 2);
        assert!(count_digits(11) == 2);
        assert!(count_digits(1111) == 4);
        assert!(count_digits(111) == 3);
        assert!(count_digits(1234567) == 7);
    }

    #[test]
    fn test_split_digits_in_half() {
        assert!(split_digits_in_half(11) == (1, 1));
        assert!(split_digits_in_half(22) == (2, 2));
        assert!(split_digits_in_half(1000) == (10, 0));
        assert!(split_digits_in_half(10) == (1, 0));
    }
}
