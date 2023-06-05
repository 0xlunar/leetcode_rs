use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut result: Vec<Vec<String>> = Vec::new();
        let sorted = strs
            .iter()
            .map(|x| Solution::sort_string(x.to_string()))
            .collect::<Vec<String>>();

        let mut indexes: HashMap<String, usize> = HashMap::new();
        for i in 0..strs.len() {
            match indexes.get(&sorted[i]) {
                Some(idx) => result.as_mut_slice()[*idx].push(strs[i].to_owned()),
                None => {
                    indexes.insert(sorted[i].to_owned(), result.len());
                    result.push(vec![strs[i].to_owned()]);
                }
            }
        }

        result
    }

    fn sort_string(str: String) -> String {
        let mut chars_one: Vec<char> = str.chars().collect::<Vec<char>>();
        chars_one.sort();
        chars_one.into_iter().collect::<String>()
    }
}

struct Solution {}
