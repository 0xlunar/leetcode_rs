use std::collections::{HashMap, HashSet};
impl Solution {
    // Need to revise this later.
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut cols: Vec<HashSet<char>> = Vec::new();
        let mut rows: Vec<HashSet<char>> = Vec::new();
        let mut squares: HashMap<(usize, usize), HashSet<char>> = HashMap::new();

        for x in 0..9 {
            for y in 0..9 {
                if cols.len() == x {
                    cols.push(HashSet::new());
                }

                if rows.len() == y {
                    rows.push(HashSet::new());
                }

                let square_idx = match x {
                    0 => match y {
                        0 => (0, 0),
                        _ => (0, y / 3),
                    },
                    _ => match y {
                        0 => (x / 3, 0),
                        _ => (x / 3, y / 3),
                    },
                };

                if !squares.contains_key(&square_idx) {
                    squares.insert(square_idx, HashSet::new());
                }

                if board[x][y] == '.' {
                    continue;
                }

                if cols[x].contains(&board[x][y])
                    || rows[y].contains(&board[x][y])
                    || squares[&square_idx].contains(&board[x][y])
                {
                    return false;
                }

                cols[x].insert(board[x][y]);
                rows[y].insert(board[x][y]);
                squares.get_mut(&square_idx).unwrap().insert(board[x][y]);
            }
        }

        true
    }
}
struct Solution {}
