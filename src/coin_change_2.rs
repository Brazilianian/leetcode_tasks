pub struct Solution {
}

impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let filtered_coins: Vec<&i32> = coins.iter()
            .filter(|c| c <= &&amount)
            .collect();


        return 0;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_equals() {
        Solution::change(5, vec![1,2,3,4,5,6,7,8,9]);
    }
}