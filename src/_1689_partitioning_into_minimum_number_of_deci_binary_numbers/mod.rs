#[allow(dead_code)]
pub fn min_partitions(n: String) -> i32 {
    n.chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .max()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(min_partitions("32".to_string()), 3);
    }
    #[test]
    fn test_2() {
        assert_eq!(min_partitions("82734".to_string()), 8);
    }
}
