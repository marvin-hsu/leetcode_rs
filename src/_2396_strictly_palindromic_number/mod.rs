pub fn is_strictly_palindromic(n: i32) -> bool {
    for i in 2..n - 1 {
        let mut s = String::new();
        let mut num = n;

        loop {
            let digit = num % i;

            s.push_str(&digit.to_string());

            num = num / i;

            if num == 0 {
                break;
            }
        }

        if is_palindromic(&s) == false {
            return false;
        }
    }
    true
}

fn is_palindromic(input: &str) -> bool {
    let mut start = 0;
    let mut end = input.len() - 1;
    let chars = input.chars().collect::<Vec<char>>();

    while start < end {
        if chars[start] != chars[end] {
            return false;
        }

        start += 1;
        end -= 1;
    }

    true
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
