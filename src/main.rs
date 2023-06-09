mod contains_duplicate;
mod group_anagrams;
mod is_anagram;
mod product_of_array_except_self;
mod top_k_frequent_elements;
mod two_sum;
mod valid_sudoku;

fn main() {
    let board = vec![
        vec!['.', '.', '.', '.', '.', '.', '5', '.', '.'],
        vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
        vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
        vec!['9', '3', '.', '.', '2', '.', '4', '.', '.'],
        vec!['.', '.', '7', '.', '.', '.', '3', '.', '.'],
        vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
        vec!['.', '.', '.', '3', '4', '.', '.', '.', '.'],
        vec!['.', '.', '.', '.', '.', '3', '.', '.', '.'],
        vec!['.', '.', '.', '.', '.', '5', '2', '.', '.'],
    ];

    let result = valid_sudoku::Solution::is_valid_sudoku(board);

    println!("Sudoku Valid: {}", result);
}
