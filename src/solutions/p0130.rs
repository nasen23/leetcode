fn dfs(board: &mut Vec<Vec<char>>, i: usize, j: usize) {
    board[i][j] = '-';
    if i > 0 && board[i - 1][j] == 'O' {
        dfs(board, i - 1, j);
    }
    if j > 0 && board[i][j - 1] == 'O' {
        dfs(board, i, j - 1);
    }
    if i < board.len() - 1 && board[i + 1][j] == 'O' {
        dfs(board, i + 1, j);
    }
    if j < board[0].len() - 1 && board[i][j + 1] == 'O' {
        dfs(board, i, j + 1);
    }
}

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        if board.is_empty() || board[0].is_empty() {
            return;
        }
        for i in 0..board[0].len() {
            if board[0][i] == 'O' {
                dfs(board, 0, i);
            }
            if board[board.len() - 1][i] == 'O' {
                dfs(board, board.len() - 1, i);
            }
        }
        for i in 0..board.len() {
            if board[i][0] == 'O' {
                dfs(board, i, 0);
            }
            if board[i][board[0].len() - 1] == 'O' {
                dfs(board, i, board[0].len() - 1);
            }
        }
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if board[i][j] == 'O' {
                    board[i][j] = 'X';
                } else if board[i][j] == '-' {
                    board[i][j] = 'O';
                }
            }
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn surrounded_regions() {
        let mut b = vec_vec![
            ['X', 'X', 'X', 'X'],
            ['X', 'O', 'O', 'X'],
            ['X', 'X', 'O', 'X'],
            ['X', 'O', 'X', 'X']
        ];
        Solution::solve(&mut b);
        assert_eq!(
            b,
            vec_vec![
                ['X', 'X', 'X', 'X'],
                ['X', 'X', 'X', 'X'],
                ['X', 'X', 'X', 'X'],
                ['X', 'O', 'X', 'X']
            ]
        );
        let mut b = vec_vec![
            ['X', 'O', 'X', 'O', 'X', 'O'],
            ['O', 'X', 'O', 'X', 'O', 'X'],
            ['X', 'O', 'X', 'O', 'X', 'O'],
            ['O', 'X', 'O', 'X', 'O', 'X']
        ];
        Solution::solve(&mut b);
        assert_eq!(
            b,
            vec_vec![
                ['X', 'O', 'X', 'O', 'X', 'O'],
                ['O', 'X', 'X', 'X', 'X', 'X'],
                ['X', 'X', 'X', 'X', 'X', 'O'],
                ['O', 'X', 'O', 'X', 'O', 'X']
            ]
        );
        let mut b = vec_vec![
            ['O', 'O', 'O', 'O', 'X', 'X'],
            ['O', 'O', 'O', 'O', 'O', 'O'],
            ['O', 'X', 'O', 'X', 'O', 'O'],
            ['O', 'X', 'O', 'O', 'X', 'O'],
            ['O', 'X', 'O', 'X', 'O', 'O'],
            ['O', 'X', 'O', 'O', 'O', 'O']
        ];
        Solution::solve(&mut b);
        assert_eq!(
            b,
            vec_vec![
                ['O', 'O', 'O', 'O', 'X', 'X'],
                ['O', 'O', 'O', 'O', 'O', 'O'],
                ['O', 'X', 'O', 'X', 'O', 'O'],
                ['O', 'X', 'O', 'O', 'X', 'O'],
                ['O', 'X', 'O', 'X', 'O', 'O'],
                ['O', 'X', 'O', 'O', 'O', 'O']
            ]
        );
    }
}
