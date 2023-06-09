mod contains_duplicate;
mod group_anagrams;
mod is_anagram;
mod longest_consecutive;
mod product_of_array_except_self;
mod top_k_frequent_elements;
mod two_sum;
mod valid_sudoku;

fn main() {
    let nums = vec![1, 2, 0, 1];

    let result = longest_consecutive::Solution::longest_consecutive(nums);

    println!("Longest Consecutive: {}", result);
}
