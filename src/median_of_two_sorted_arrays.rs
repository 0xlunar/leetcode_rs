impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut combined = nums1.to_owned();
        combined.append(&mut nums2.to_owned());
        combined.sort_unstable();
        let middle = ((combined.len() as f64) - 1.0) / 2.0;

        if middle % 1.0 > 0.0 {
            let left = middle.floor() as usize;
            let right = middle.ceil() as usize;
            ((combined.as_slice()[left] + combined.as_slice()[right]) as f64 / 2.0) as f64
        } else {
            combined.as_slice()[middle as usize] as f64
        }
    }
}

pub struct Solution {}
