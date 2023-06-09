use std::collections::HashMap;
impl Solution {
    // https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen: HashMap<i32, i32> = HashMap::new();

        for i in 0..numbers.len() {
            match seen.get(&(target - numbers[i])) {
                Some(n) => return vec![*n + 1, i as i32 + 1],
                None => seen.insert(numbers[i], i.to_owned() as i32),
            };
        }

        vec![1, 1]
    }
}

struct Solution {}
