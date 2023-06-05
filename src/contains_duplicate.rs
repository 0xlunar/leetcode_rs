use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut uniq = HashSet::new();
        nums.iter().all(move |n| uniq.insert(n))
    }
}
