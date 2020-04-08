struct Solution;

impl Solution {
    pub fn tictactoe(board: Vec<String>) -> String {
        for row in &board {
            if row.as_bytes().into_iter().all(|&b| b == b'X') {
                return "X".into();
            }
            if row.as_bytes().into_iter().all(|&b| b == b'O') {
                return "O".into();
            }
        }

        for c in 0..board.len() {
            if (0..board.len())
                .into_iter()
                .all(|r| board[r].as_bytes()[c] == b'X')
            {
                return "X".into();
            }
            if (0..board.len())
                .into_iter()
                .all(|r| board[r].as_bytes()[c] == b'O')
            {
                return "O".into();
            }
        }

        if (0..board.len())
            .into_iter()
            .all(|x| board[x].as_bytes()[x] == b'X')
        {
            return "X".into();
        }
        if (0..board.len())
            .into_iter()
            .all(|x| board[x].as_bytes()[x] == b'O')
        {
            return "O".into();
        }
        if (0..board.len())
            .into_iter()
            .all(|x| board[x].as_bytes()[board.len() - x - 1] == b'X')
        {
            return "X".into();
        }
        if (0..board.len())
            .into_iter()
            .all(|x| board[x].as_bytes()[board.len() - x - 1] == b'O')
        {
            return "O".into();
        }

        for row in board {
            if row.as_bytes().into_iter().any(|&b| b == b' ') {
                return "Pending".into();
            }
        }

        "Draw".into()
    }
}

mod tests {
    use super::Solution;
    use crate::vec_str;

    #[test]
    fn test() {
        assert_eq!(Solution::tictactoe(vec_str!["O X", " XO", "X O"]), "X");
        assert_eq!(Solution::tictactoe(vec_str!["OOX", "XXO", "OXO"]), "Draw");
        assert_eq!(
            Solution::tictactoe(vec_str!["OOX", "XXO", "OX "]),
            "Pending"
        );
    }
}
