struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let roman_ones = ['I', 'X', 'C', 'M'];
        let roman_fives = ['V', 'L', 'D', ' '];

        let mut integer: i32 = 0;
        let char_array: Vec<char> = s.chars().collect();
        let mut i = 0;
        while i < char_array.len() {
            if let Some(index) = roman_fives.iter().position(|r| r == &char_array[i]) {
                integer += 5 * 10_i32.pow(index as u32)
            } else if let Some(index) = roman_ones.iter().position(|r| r == &char_array[i]) {
                if i == char_array.len() - 1 {
                    integer += 10_i32.pow(index as u32);
                } else {
                    if let Some(v) = roman_fives.get(index) {
                        if char_array[i + 1] == *v {
                            integer = integer + 5 * 10_i32.pow(index as u32) - 10_i32.pow(index as u32);
                            i += 2;
                            continue;
                        }
                    }
                    if let Some(v) = roman_ones.get(index + 1) {
                        if char_array[i + 1] == *v {
                            integer = integer + 10_i32.pow((index + 1) as u32) - 10_i32.pow(index as u32);
                            i += 2;
                            continue;
                        }
                    }
                    integer += 10_i32.pow(index as u32);
                }
            } else {
                panic!("Unknown roman number");
            }
            i += 1;
        }
        return integer;
    }
}

#[cfg(test)]
mod test {
    use crate::roman_to_integer::Solution;

    #[test]
    fn should_convert_from_roman_to_integer_1() {
        assert_eq!(Solution::roman_to_int(String::from("III")), 3);
    }

    #[test]
    fn should_convert_from_roman_to_integer_2() {
        assert_eq!(Solution::roman_to_int(String::from("LVIII")), 58);
    }

    #[test]
    fn should_convert_from_roman_to_integer_3() {
        assert_eq!(Solution::roman_to_int(String::from("MCMXCIV")), 1994);
    }

    #[test]
    fn should_convert_from_roman_to_integer_4() {
        assert_eq!(Solution::roman_to_int(String::from("MCDLXXVI")), 1476);
    }
}