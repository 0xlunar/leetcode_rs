impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut cleaned = s.to_owned();
        cleaned.retain(|c| c.is_alphanumeric());
        let cleaned = cleaned.to_ascii_lowercase();

        let left: String = cleaned
            .chars()
            .enumerate()
            .filter_map(|i| {
                if i.0 < cleaned.len() / 2 {
                    Some(i.1)
                } else {
                    None
                }
            })
            .collect();

        let right: String = cleaned
            .chars()
            .rev()
            .enumerate()
            .filter_map(|i| {
                if i.0 < cleaned.len() / 2 {
                    Some(i.1)
                } else {
                    None
                }
            })
            .collect::<String>();

        left == right
    }
}

pub struct Solution {}
