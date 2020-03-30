use std::net::Ipv6Addr;

struct Solution;

impl Solution {
    pub fn valid_ip_address(ip: String) -> String {
        if is_ipv4(&ip) {
            "IPv4"
        } else if ip.parse::<Ipv6Addr>().is_ok() {
            "IPv6"
        } else {
            "Neither"
        }
        .to_string()
    }
}

fn is_ipv4(ip: &str) -> bool {
    let nums: Vec<_> = ip.split(|x| x == '.').collect();
    if nums.len() != 4 {
        return false;
    }

    for n_slice in nums {
        if n_slice.len() == 0 || n_slice.len() > 3 {
            return false;
        }

        if n_slice.chars().nth(0).unwrap() == '0' && n_slice.len() != 1
            || n_slice.chars().any(|x| x < '0' || x > '9')
            || n_slice.parse::<usize>().unwrap() > 255
        {
            return false;
        }
    }

    true
}
