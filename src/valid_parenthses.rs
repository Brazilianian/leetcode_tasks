use std::collections::{HashMap, LinkedList};

struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        if s.is_empty() {
            true;
        }

        let mut expected_close_bracket = Vec::new();

        for char in s.chars() {
            match char {
                '(' | '{' | '[' => {
                    expected_close_bracket.push_front(char);
                },
                ')' | '}' | ']' => {
                    match expected_close_bracket.pop_front() {
                        Some('(') if char == ')' => (),
                        Some('[') if char == ']' => (),
                        Some('{') if char == '}' => (),
                        _ => return false
                    }

                },
                _ => return false
            }
        }

        return expected_close_bracket.is_empty()
    }
}

#[cfg(test)]
mod test {
    use crate::valid_parenthses::Solution;

    #[test]
    fn parenthness_must_be_valid_1() {
        assert_eq!(Solution::is_valid("()".to_string()), true);
    }

    #[test]
    fn parenthness_must_be_valid_2() {
        assert_eq!(Solution::is_valid("()[]{}".to_string()), true);
    }

    #[test]
    fn parenthness_must_be_valid_3() {
        assert_eq!(Solution::is_valid("(]".to_string()), false);
    }
}