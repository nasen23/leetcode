impl Solution {
    pub fn maximum_binary_string(binary: String) -> String {
        let first = binary.find('0');
        if let Some(first) = first {
            let nzeros = binary.chars().filter(|&c| c == '0').count();
            let mut bytes = vec![b'1'; binary.len()];
            bytes[first + nzeros - 1] = b'0';
            unsafe { String::from_utf8_unchecked(bytes) }
        } else {
            binary
        }
    }
}

struct Solution;
