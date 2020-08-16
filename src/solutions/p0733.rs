impl Solution {
    pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>> {
        let (sr, sc) = (sr as usize, sc as usize);
        if new_color != image[sr][sc] {
            let mut st = vec![(sr, sc)];
            while let Some((i, j)) = st.pop() {
                let orig = image[i][j];
                image[i][j] = new_color;
                if i > 0 && image[i - 1][j] == orig && image[i - 1][j] != new_color {
                    st.push((i - 1, j));
                }
                if j > 0 && image[i][j - 1] == orig && image[i][j - 1] != new_color {
                    st.push((i, j - 1));
                }
                if i < image.len() - 1 && image[i + 1][j] == orig && image[i + 1][j] != new_color {
                    st.push((i + 1, j));
                }
                if j < image[0].len() - 1 && image[i][j + 1] == orig && image[i][j + 1] != new_color
                {
                    st.push((i, j + 1));
                }
            }
        }
        image
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn flood_fill() {
        assert_eq!(
            Solution::flood_fill(vec_vec![[1, 1, 1], [1, 1, 0], [1, 0, 1]], 1, 1, 2),
            vec_vec![[2, 2, 2], [2, 2, 0], [2, 0, 1]]
        );
        assert_eq!(
            Solution::flood_fill(vec_vec![[0, 0, 0], [0, 1, 1]], 1, 1, 1),
            vec_vec![[0, 0, 0], [0, 1, 1]]
        );
    }
}
