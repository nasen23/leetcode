impl Solution {
    pub fn largest_rectangle_area(mut heights: Vec<i32>) -> i32 {
        let mut h = vec![0];
        h.append(&mut heights);
        h.push(0);
        let mut res = 0;
        let mut st = vec![];
        for i in 0..h.len() {
            while let Some(&top) = st.last() {
                if h[top] > h[i] {
                    st.pop();
                    res = res.max((i - *st.last().unwrap() - 1) as i32 * h[top]);
                } else {
                    break;
                }
            }
            st.push(i);
        }
        res
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn largest_rect_in_diagram() {
        assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
        assert_eq!(
            Solution::largest_rectangle_area((0..20000).collect()),
            100000000
        );
    }
}
