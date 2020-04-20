use std::collections::VecDeque;

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let x = grid.len();
        if x == 0 {
            return 0;
        }
        let y = grid[0].len();
        let mut grid = grid;

        let mut count = 0;
        for i in 0..x {
            for j in 0..y {
                if grid[i][j] == '1' {
                    // start bfs
                    let mut queue = VecDeque::new();
                    queue.push_back((i, j));
                    grid[i][j] = '0';
                    while let Some((i, j)) = queue.pop_front() {
                        if i > 0 && grid[i - 1][j] == '1' {
                            queue.push_back((i - 1, j));
                            grid[i - 1][j] = '0';
                        }
                        if j > 0 && grid[i][j - 1] == '1' {
                            queue.push_back((i, j - 1));
                            grid[i][j - 1] = '0';
                        }
                        if i < x - 1 && grid[i + 1][j] == '1' {
                            queue.push_back((i + 1, j));
                            grid[i + 1][j] = '0';
                        }
                        if j < y - 1 && grid[i][j + 1] == '1' {
                            queue.push_back((i, j + 1));
                            grid[i][j + 1] = '0';
                        }
                    }

                    count += 1;
                }
            }
        }

        count
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn number_of_islands() {
        let ve_str_to_chars = |ve: Vec<&str>| {
            ve.into_iter()
                .map(|s| s.chars().collect::<Vec<_>>())
                .collect::<Vec<Vec<char>>>()
        };
        assert_eq!(
            Solution::num_islands(ve_str_to_chars(vec!["10", "01"])),
            2
        );
        assert_eq!(
            Solution::num_islands(ve_str_to_chars(vec!["11110", "11010", "11000", "00000"])),
            1
        );
    }
}
