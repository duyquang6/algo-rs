use std::collections::{HashMap, VecDeque};

struct Solution;

const MOD: i64 = 1e9 as i64 + 7;
impl Solution {
    pub fn count_good_numbers(n: i64) -> i32 {
        return ((fast_exponential(5, (n + 1) >> 1) * fast_exponential(4, n >> 1)) % MOD) as i32;
    }
}

fn fast_exponential(base: i32, mut exp: i64) -> i64 {
    if exp == 0 {
        return 1;
    }
    let mut ans = 1i64;
    let mut base = (base as i64) % MOD;
    while exp > 0 {
        if exp & 1 == 1 {
            ans = (ans * base) % MOD;
        }
        base = (base * base) % MOD;
        exp >>= 1;
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_good_numbers() {
        assert_eq!(Solution::count_good_numbers(50), 564908303);
    }
}
