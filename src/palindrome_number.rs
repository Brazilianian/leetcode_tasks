struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let digits: Vec<char> = x.to_string().chars().collect();

        for i in 0..digits.len() {
            if i == digits.len() / 2 {
                return true;
            }

            if digits[i] != digits[digits.len() - i - 1] {
                return false;
            }
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_assert_if_number_is_palindrome() {
        assert_eq!(Solution::is_palindrome(121), true);
        assert_eq!(Solution::is_palindrome(1211), false);
        assert_eq!(Solution::is_palindrome(1221), true);
        assert_eq!(Solution::is_palindrome(11), true);
        assert_eq!(Solution::is_palindrome(1), true);
        assert_eq!(Solution::is_palindrome(-1), false);
    }
}