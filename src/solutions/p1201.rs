impl Solution {
    pub fn nth_ugly_number(n: i32, a: i32, b: i32, c: i32) -> i32 {
        let (a, b, c, n) = (a as i64, b as i64, c as i64, n as i64);
        let lcm_ab = lcm(a, b);
        let lcm_bc = lcm(b, c);
        let lcm_ac = lcm(a, c);
        let lcm_abc = lcm(lcm_ab, c);
        let mut l = a.min(b).min(c);
        let mut r = l * n;
        while l <= r {
            let mid = l + (r - l) / 2;
            let c = mid / a + mid / b + mid / c - mid / lcm_ab - mid / lcm_ac - mid / lcm_bc
                + mid / lcm_abc;
            if c >= n {
                r = mid - 1;
            } else {
                l = mid + 1;
            }
        }
        l as i32
    }
}

fn lcm(mut a: i64, mut b: i64) -> i64 {
    let m = a * b;
    while b > 0 {
        let tmp = a % b;
        a = b;
        b = tmp;
    }
    m / a
}

struct Solution;
