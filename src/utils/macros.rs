#[macro_export]
macro_rules! sorted {
    ($e:expr) => {{
        let mut x = $e;
        x.sort();
        x
    }};
}

#[macro_export]
macro_rules! vec_str {
    [$($e:expr), *] => {vec![$($e.to_owned()), *]};
}

#[macro_export]
macro_rules! vec_vec {
    [$($e:expr), *] => {vec![$($e.to_vec()), *]};
}

#[macro_export]
macro_rules! linkedlist {
    () => {
        None
    };
    ($($e:expr), *) => {
        {
            let mut head = Box::new($crate::ListNode::new(0));
            let mut ref_head = &mut head;

            $(
            ref_head.next = Some(Box::new($crate::ListNode::new($e)));
            ref_head = ref_head.next.as_mut().unwrap();
            )*

            let _ = ref_head;
            head.next
        }
    }
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
