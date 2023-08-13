struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut common_prefix = String::new();
        if let Some(first) = strs.get(0) {
            for i in 0..first.len() {
                for str in &strs {
                    if let Some(char) = str.chars().nth(i) {
                        if char != first.chars().nth(i).unwrap() {
                            return common_prefix;
                        }
                    } else {
                        return common_prefix;
                    }
                }
                common_prefix += &first.chars().nth(i).unwrap().to_string();
            }
        }
        return common_prefix;
    }
}

#[cfg(test)]
mod test {
    use crate::longest_common_prefix::Solution;

    #[test]
    fn should_find_longest_common_prefix_1() {
        assert_eq!(Solution::longest_common_prefix(vec![
            String::from("flower"),
            String::from("flow"),
            String::from("flight")]), String::from("fl"));
    }

    #[test]
    fn should_find_longest_common_prefix_2() {
        assert_eq!(Solution::longest_common_prefix(vec![
            String::from("dog"),
            String::from("racecar"),
            String::from("car")]), String::from(""));
    }

    #[test]
    fn should_find_longest_common_prefix_3() {
        assert_eq!(Solution::longest_common_prefix(vec![
            String::from("ab"),
            String::from("a")]), String::from("a"));
    }
}