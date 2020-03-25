use std::cmp::min;

struct Solution;

impl Solution {
    pub fn surface_area(grid: Vec<Vec<i32>>) -> i32 {
        let mut map = vec![vec![false; 50]; 50];
        let mut area = 0;

        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                map[i][j] = true;
                if grid[i][j] == 0 {
                    continue;
                }
                let mut delta = grid[i][j] * 4 + 2;
                if i > 0 && map[i - 1][j] {
                    delta -= 2 * min(grid[i - 1][j], grid[i][j]);
                }
                if j > 0 && map[i][j - 1] {
                    delta -= 2 * min(grid[i][j - 1], grid[i][j]);
                }

                area += delta;
            }
        }

        area
    }
}

#[test]
fn case1() {
    let grid = vec![vec![2]];
    assert_eq!(Solution::surface_area(grid), 10);
}

#[test]
fn case2() {
    let grid = vec![vec![1, 2], vec![3, 4]];
    assert_eq!(Solution::surface_area(grid), 34);
}
