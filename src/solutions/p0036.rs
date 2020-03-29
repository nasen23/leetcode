use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut row_set = HashSet::new();
        let mut block_set = vec![HashSet::new(); 3];
        let mut col_set = vec![HashSet::new(); 9];

        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] != '.' {
                    let c = board[i][j];
                    if !row_set.insert(c) {
                        return false;
                    }
                    if !block_set[j / 3].insert(c) {
                        return false;
                    }
                    if !col_set[j].insert(c) {
                        return false;
                    }
                }
            }
            row_set.clear();
            if i % 3 == 2 {
                for set in &mut block_set {
                    set.clear();
                }
            }
        }

        true
    }
}

#[test]
fn case1() {
    let board = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];

    assert!(Solution::is_valid_sudoku(board));
}

#[test]
fn case2() {
    let board = vec![
        vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];
    assert!(!Solution::is_valid_sudoku(board));
}
