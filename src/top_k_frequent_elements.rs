use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut frequencies: HashMap<i32, i32> = HashMap::with_capacity(nums.len());

        for i in 0..nums.len() {
            match frequencies.get(&nums[i]) {
                Some(n) => frequencies.insert(nums[i], n + 1),
                None => frequencies.insert(nums[i], 1),
            };
        }

        let mut sorted: Vec<_> = frequencies.iter().collect();
        sorted.sort_by(|a, b| b.1.cmp(a.1));
        let result: Vec<i32> = sorted.iter().map(|x| x.0.to_owned()).collect();
        result.iter().take(k as usize).cloned().collect()
    }
}

struct Solution {}
