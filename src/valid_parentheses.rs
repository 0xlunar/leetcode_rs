use std::collections::HashMap;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut partners: HashMap<char, char> = HashMap::new();
        let mut stack: Vec<char> = Vec::new();

        partners.insert(']', '[');
        partners.insert(')', '(');
        partners.insert('}', '{');

        for c in s.chars() {
            if !partners.contains_key(&c) {
                stack.push(c);
                continue;
            }
            if stack.len() == 0 || stack.last().unwrap() != partners.get(&c).unwrap() {
                return false;
            }
            stack.pop();
        }

        stack.len() == 0
    }
}

pub struct Solution {}
