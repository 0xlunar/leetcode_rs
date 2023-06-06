use std::collections::HashMap;

impl Solution {
    // First element won't be multiplied by the last element, so we need to reverse and do it again.
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![1; nums.len()];
        (0..nums.len()).fold(1, |mut sum: i32, i| {
            result[i] = sum;
            sum = sum * nums[i];
            sum
        });

        (0..nums.len()).rev().fold(1, |mut sum: i32, i| {
            result[i] = result[i] * sum;
            sum = sum * nums[i];
            sum
        });

        result
    }
}

struct Solution {}
