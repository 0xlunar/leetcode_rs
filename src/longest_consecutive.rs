impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut longest: i32 = 0;
        let mut current: i32 = 0;

        let mut sorted = nums.to_owned();
        sorted.sort();
        sorted.dedup();

        for i in 0..sorted.len() {
            if i == 0 || sorted[i - 1] + 1 == sorted[i] {
                current += 1;
            } else {
                current = 1;
            }

            if current > longest {
                longest = current;
            }
        }

        longest
    }
}

pub struct Solution {}
