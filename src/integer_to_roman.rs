struct Solution;

const UNITS: &[(&str, i32)] = &[
    ("M", 1000),
    ("CM", 900),
    ("D", 500),
    ("CD", 400),
    ("C", 100),
    ("XC", 90),
    ("L", 50),
    ("XL", 40),
    ("X", 10),
    ("IX", 9),
    ("V", 5),
    ("IV", 4),
    ("I", 1),
];

impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
      return String::new();
    }
}

#[cfg(test)]
mod test {
    use crate::integer_to_roman::Solution;

    #[test]
    fn should_parse_roman_to_integer() {
        assert_eq!(Solution::int_to_roman(3), "III")
    }
}