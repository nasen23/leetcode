struct Solution;

impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if board[i][j] & 1 == 1 {
                    let mut alive_cell = 0;
                    for a in i.checked_sub(1).unwrap_or(0)..i + 2 {
                        for b in j.checked_sub(1).unwrap_or(0)..j + 2 {
                            if a < board.len()
                                && b < board[0].len()
                                && !(a == i && b == j)
                                && board[a][b] & 1 == 1
                            {
                                alive_cell += 1;
                            }
                        }
                    }

                    if alive_cell == 2 || alive_cell == 3 {
                        board[i][j] |= 0b10;
                    }
                } else {
                    // dead cell
                    let mut alive_cell = 0;
                    for a in i.checked_sub(1).unwrap_or(0)..i + 2 {
                        for b in j.checked_sub(1).unwrap_or(0)..j + 2 {
                            if a < board.len()
                                && b < board[0].len()
                                && !(a == i && b == j)
                                && board[a][b] & 1 == 1
                            {
                                alive_cell += 1;
                            }
                        }
                    }

                    if alive_cell == 3 {
                        board[i][j] |= 0b10;
                    }
                }
            }
        }

        for row in board {
            for cell in row.iter_mut() {
                *cell >>= 1;
            }
        }
    }
}

#[test]
fn test() {
    let mut input = vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1], vec![0, 0, 0]];
    let res = vec![vec![0, 0, 0], vec![1, 0, 1], vec![0, 1, 1], vec![0, 1, 0]];
    Solution::game_of_life(&mut input);
    assert_eq!(input, res);
}
