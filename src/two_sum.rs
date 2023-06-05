use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen: HashMap<i32, i32> = HashMap::new();

        for i in 0..nums.len() {
            match seen.get(&(target - nums[i])) {
                Some(n) => return vec![*n, i as i32],
                None => seen.insert(nums[i], i.to_owned() as i32),
            };
        }
        vec![]
    }
}
struct Solution {}
