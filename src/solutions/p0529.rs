use std::collections::VecDeque;

impl Solution {
    pub fn update_board(mut board: Vec<Vec<char>>, click: Vec<i32>) -> Vec<Vec<char>> {
        let (m, n) = (board.len(), board[0].len());
        let (x, y) = (click[0] as usize, click[1] as usize);
        match board[x][y] {
            'M' => {
                board[x][y] = 'X';
            }
            'E' => {
                let mut queue = VecDeque::new();
                queue.push_back((x, y));
                while let Some((x, y)) = queue.pop_front() {
                    if board[x][y] != 'E' {
                        continue;
                    }
                    let mut c = 0;
                    let mut tmp = VecDeque::new();
                    for i in 0..3 {
                        for j in 0..3 {
                            if i != 1 || j != 1 {
                                if x + i > 0 && y + j > 0 && x + i < m + 1 && y + j < n + 1 {
                                    let (xx, yy) = (x + i - 1, y + j - 1);
                                    if board[xx][yy] == 'M' {
                                        c += 1;
                                    } else if board[xx][yy] == 'E' {
                                        tmp.push_back((xx, yy));
                                    }
                                }
                            }
                        }
                    }
                    if c == 0 {
                        board[x][y] = 'B';
                        queue.append(&mut tmp);
                    } else {
                        board[x][y] = (b'0' + c) as char;
                    }
                }
            }
            _ => {}
        }
        board
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn minesweeper() {
        assert_eq!(
            Solution::update_board(
                vec_vec![
                    ['E', 'E', 'E', 'E', 'E'],
                    ['E', 'E', 'M', 'E', 'E'],
                    ['E', 'E', 'E', 'E', 'E'],
                    ['E', 'E', 'E', 'E', 'E']
                ],
                vec![3, 0]
            ),
            vec_vec![
                ['B', '1', 'E', '1', 'B'],
                ['B', '1', 'M', '1', 'B'],
                ['B', '1', '1', '1', 'B'],
                ['B', 'B', 'B', 'B', 'B']
            ]
        );
        assert_eq!(
            Solution::update_board(
                vec_vec![
                    ['B', '1', 'E', '1', 'B'],
                    ['B', '1', 'M', '1', 'B'],
                    ['B', '1', '1', '1', 'B'],
                    ['B', 'B', 'B', 'B', 'B']
                ],
                vec![1, 2]
            ),
            vec_vec![
                ['B', '1', 'E', '1', 'B'],
                ['B', '1', 'X', '1', 'B'],
                ['B', '1', '1', '1', 'B'],
                ['B', 'B', 'B', 'B', 'B']
            ]
        );
        assert_eq!(
            Solution::update_board(
                vec_vec![
                    ['E', 'E', 'E', 'E', 'E'],
                    ['E', 'E', 'E', 'E', 'E'],
                    ['E', 'E', 'E', 'E', 'E'],
                    ['E', 'E', 'E', 'E', 'E']
                ],
                vec![1, 2]
            ),
            vec_vec![
                ['B', 'B', 'B', 'B', 'B'],
                ['B', 'B', 'B', 'B', 'B'],
                ['B', 'B', 'B', 'B', 'B'],
                ['B', 'B', 'B', 'B', 'B']
            ]
        );
        assert_eq!(
            Solution::update_board(vec![vec!['E'; 100]; 100], vec![1, 2]),
            vec![vec!['B'; 100]; 100]
        );
    }
}
