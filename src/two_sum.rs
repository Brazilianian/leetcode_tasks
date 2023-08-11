use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut nums_hash_map: HashMap<i32, usize> = HashMap::new();

        for i in 0..nums.len() {
            let complement = target - nums[i];
            if nums_hash_map.contains_key(&complement) {
                let j = nums_hash_map.get(&complement).unwrap();
                if j == &i {
                    continue;
                }

                return vec![i as i32, *j as i32]
            }

            nums_hash_map.insert(nums[i], i);
        }

        return vec![];
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        let nums = vec![3, 2, 4];
        let nums_clone = nums.clone();
        let target = 5;
        let vec_solution = Solution::two_sum(nums, target);
        assert_eq!(nums_clone[vec_solution[0] as usize] + nums_clone[vec_solution[1] as usize], target);
    }
}