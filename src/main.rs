mod contains_duplicate;
mod group_anagrams;
mod is_anagram;
mod longest_consecutive;
mod median_of_two_sorted_arrays;
mod min_stack;
mod product_of_array_except_self;
mod reverse_polish_notation;
mod top_k_frequent_elements;
mod two_sum;
mod two_sum_ii;
mod valid_palindrome;
mod valid_parentheses;
mod valid_sudoku;

fn main() {
    let input1 = vec![1, 3];
    let input2 = vec![2];

    let result = median_of_two_sorted_arrays::Solution::find_median_sorted_arrays(input1, input2);

    println!("Result: {:?}", result);
}
