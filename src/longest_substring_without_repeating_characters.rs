struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.len() == 0 {
            return 0;
        }

        let mut longest_substring: String = String::new();
        let char_array = s.chars().collect::<Vec<char>>();

        for i in 0..s.len() {
            let mut substring = char_array.get(i).unwrap().to_string();
            for j in i + 1..s.len() {
                if substring.contains(&char_array.get(j).unwrap().to_string()) {
                    break;
                }
                substring += &char_array.get(j).unwrap().to_string();
            }
            if longest_substring.len() < substring.len() {
                longest_substring = substring;
            }
        }
        return longest_substring.len() as i32;
    }
}

#[cfg(test)]
mod test {
    use crate::longest_substring_without_repeating_characters::Solution;

    #[test]
    fn should_find_length_of_longest_substring_without_repeating_characters_1() {
        assert_eq!(Solution::length_of_longest_substring("abcabcbb".to_string()), 3)
    }

    #[test]
    fn should_find_length_of_longest_substring_without_repeating_characters_2() {
        assert_eq!(Solution::length_of_longest_substring("bbbbb".to_string()), 1)
    }

    #[test]
    fn should_find_length_of_longest_substring_without_repeating_characters_3() {
        assert_eq!(Solution::length_of_longest_substring("pwwkew".to_string()), 3)
    }

    #[test]
    fn should_find_length_of_longest_substring_without_repeating_characters_4() {
        assert_eq!(Solution::length_of_longest_substring(" ".to_string()), 1)
    }

    #[test]
    fn should_find_length_of_longest_substring_without_repeating_characters_5() {
        assert_eq!(Solution::length_of_longest_substring("au".to_string()), 2)
    }
}