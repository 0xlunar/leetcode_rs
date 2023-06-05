impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let s = Solution::sort_string(s);
        let t = Solution::sort_string(t);
        t == s
    }

    fn sort_string(str: String) -> String {
        let mut chars_one: Vec<char> = str.chars().collect::<Vec<char>>();
        chars_one.sort();
        chars_one.into_iter().collect::<String>()
    }
}

struct Solution {}
