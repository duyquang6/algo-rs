use std::collections::HashMap;

impl Solution {
    pub fn four_key_keyboard(n: i32) -> i32 {
        recur(n)
    }
}

fn recur(n: i32) -> i32 {
    if n == 1 {
        return 1;
    }
    // pick A
    let mut ans = recur(n - 1) + 1;

    for i in 1..=n - 3 {
        ans = ans.max(recur(i) * (n - i - 1));
    }

    ans
}

struct Solution;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_four_key_keyboard() {
        assert_eq!(Solution::four_key_keyboard(7), 9);
        assert_eq!(Solution::four_key_keyboard(3), 3);
    }
}
