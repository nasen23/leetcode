#[macro_export]
macro_rules! vec_str {
    [$($e:expr), *] => {vec![$($e.to_owned()), *]};
}

#[macro_export]
macro_rules! vec_vec {
	[$($e:expr), *] => {vec![$($e.to_vec()), *]};
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_vec_vec() {
        assert_eq!(vec_vec![[1, 2], [3, 4]], vec![vec![1, 2], vec![3, 4]]);
    }

    #[test]
    fn test_vec_str() {
        assert_eq!(vec_str!["f", "fe"], vec!["f".to_owned(), "fe".to_owned()]);
    }
}
