mod contains_duplicate;
mod group_anagrams;
mod is_anagram;
mod longest_consecutive;
mod product_of_array_except_self;
mod top_k_frequent_elements;
mod two_sum;
mod two_sum_ii;
mod valid_palindrome;
mod valid_parentheses;
mod valid_sudoku;

fn main() {
    let input = "()[]{}".to_string();

    let result = valid_parentheses::Solution::is_valid(input);

    println!("Result: {:?}", result);
}
