/// 實際上這題永遠都是false
/// 因為n如果轉換為n-2進位制 => n = 1 * (n-2) ^ 1 + 2 * (n-2) ^ 0 = 12
/// 永遠不會達成回文數的條件
pub fn is_strictly_palindromic(n: i32) -> bool {
    (2..n - 1).all(|num| {
        let mut items = Vec::new();
        let mut n = n;
        while n > 0 {
            items.push(n % num);
            n /= num;
        }

        items
            .iter()
            .rev()
            .zip(items.iter())
            .take(items.len() / 2 + 1)
            .all(|(a, b)| a == b)
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_strictly_palindromic() {
        assert_eq!(is_strictly_palindromic(9), false);
        assert_eq!(is_strictly_palindromic(4), false);
    }
}
