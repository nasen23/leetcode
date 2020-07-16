impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut row = vec![vec![false; 9]; 9];
        let mut col = vec![vec![false; 9]; 9];
        let mut block = vec![vec![false; 9]; 9];
        let mut rest = vec![];
        for i in 0..9 {
            for j in 0..9 {
                match board[i][j] {
                    '.' => rest.push((i, j)),
                    _ => {
                        let n = (board[i][j] as u8 - b'1') as usize;
                        row[i][n] = true;
                        col[j][n] = true;
                        block[i / 3 * 3 + j / 3][n] = true;
                    }
                }
            }
        }
        dfs(board, &rest, &mut row, &mut col, &mut block);
    }
}

fn dfs(
    board: &mut Vec<Vec<char>>,
    rest: &[(usize, usize)],
    row: &mut Vec<Vec<bool>>,
    col: &mut Vec<Vec<bool>>,
    block: &mut Vec<Vec<bool>>,
) -> bool {
    if let Some((i, j)) = rest.first() {
        let (i, j) = (*i, *j);
        for x in 0..9 {
            if !row[i][x] && !col[j][x] && !block[i / 3 * 3 + j / 3][x] {
                row[i][x] = true;
                col[j][x] = true;
                block[i / 3 * 3 + j / 3][x] = true;
                board[i][j] = (x as u8 + b'1') as char;
                if dfs(board, &rest[1..], row, col, block) {
                    return true;
                }
                row[i][x] = false;
                col[j][x] = false;
                block[i / 3 * 3 + j / 3][x] = false;
            }
        }
        false
    } else {
        true
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn sudoku_solver() {
        let mut board = vec_vec![
            ['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            ['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            ['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            ['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            ['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            ['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            ['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            ['.', '.', '.', '.', '8', '.', '.', '7', '9']
        ];
        Solution::solve_sudoku(&mut board);
        println!("{:?}", board);
        assert!(false);
    }
}
