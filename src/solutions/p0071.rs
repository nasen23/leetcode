impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut slices = vec![];
        for dir in path.split("/") {
            match dir {
                "" | "." => {}
                ".." => {
                    slices.pop();
                }
                _ => slices.push(dir),
            }
        }
        "/".to_string() + &slices.join("/")
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simplify_path() {
        assert_eq!(Solution::simplify_path("/home/".into()), "/home");
        assert_eq!(Solution::simplify_path("/../".into()), "/");
        assert_eq!(Solution::simplify_path("/home//foo/".into()), "/home/foo");
        assert_eq!(Solution::simplify_path("/a/./b/../../c/".into()), "/c");
        assert_eq!(
            Solution::simplify_path("/a//b////c/d//././/..".into()),
            "/a/b/c"
        );
        assert_eq!(Solution::simplify_path("/a/../../b/../c//.//".into()), "/c");
    }
}
