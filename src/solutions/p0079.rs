impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let (m, n) = (board.len(), board[0].len());
        let mut vis = vec![vec![false; n]; m];
        let word = word.as_bytes();
        for i in 0..m {
            for j in 0..n {
                if board[i][j] == word[0] as char {
                    vis[i][j] = true;
                    if find_recursive(&board, &mut vis, i, j, &word[1..]) {
                        return true;
                    }
                    vis[i][j] = false;
                }
            }
        }
        false
    }
}

fn find_recursive(
    board: &Vec<Vec<char>>,
    vis: &mut Vec<Vec<bool>>,
    i: usize,
    j: usize,
    word: &[u8],
) -> bool {
    if word.is_empty() {
        return true;
    }
    if i < board.len() - 1 && !vis[i + 1][j] && board[i + 1][j] == word[0] as char {
        vis[i + 1][j] = true;
        if find_recursive(board, vis, i + 1, j, &word[1..]) {
            return true;
        }
        vis[i + 1][j] = false;
    }
    if j < board[0].len() - 1 && !vis[i][j + 1] && board[i][j + 1] == word[0] as char {
        vis[i][j + 1] = true;
        if find_recursive(board, vis, i, j + 1, &word[1..]) {
            return true;
        }
        vis[i][j + 1] = false;
    }
    if i > 0 && !vis[i - 1][j] && board[i - 1][j] == word[0] as char {
        vis[i - 1][j] = true;
        if find_recursive(board, vis, i - 1, j, &word[1..]) {
            return true;
        }
        vis[i - 1][j] = false;
    }
    if j > 0 && !vis[i][j - 1] && board[i][j - 1] == word[0] as char {
        vis[i][j - 1] = true;
        if find_recursive(board, vis, i, j - 1, &word[1..]) {
            return true;
        }
        vis[i][j - 1] = false;
    }
    false
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn word_search() {
        assert!(Solution::exist(
            vec_vec![
                ['A', 'B', 'C', 'E'],
                ['S', 'F', 'C', 'S'],
                ['A', 'D', 'E', 'E']
            ],
            "ABCCED".into()
        ));
        assert!(Solution::exist(
            vec_vec![
                ['A', 'B', 'C', 'E'],
                ['S', 'F', 'C', 'S'],
                ['A', 'D', 'E', 'E']
            ],
            "SEE".into()
        ));
        assert!(!Solution::exist(
            vec_vec![
                ['A', 'B', 'C', 'E'],
                ['S', 'F', 'C', 'S'],
                ['A', 'D', 'E', 'E']
            ],
            "ABCB".into()
        ));
        assert!(Solution::exist(
            vec_vec![
                ['A', 'B', 'C', 'E'],
                ['S', 'F', 'E', 'S'],
                ['A', 'D', 'E', 'E']
            ],
            "ABCESEEEFS".into()
        ));
        assert!(Solution::exist(vec_vec![['A']], "A".into()));
    }
}
