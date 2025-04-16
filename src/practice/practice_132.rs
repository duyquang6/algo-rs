impl Solution {
    pub fn min_cut(s: String) -> i32 {
        let mut memo = vec![-1; s.len()];
        recur(s.as_str(), &mut memo, 0usize)
    }
}

fn recur(s: &str, memo: &mut Vec<i32>, idx: usize) -> i32 {
    if idx == s.len() {
        return -1;
    }

    if memo[idx] != -1 {
        return memo[idx];
    }
    let n = s.len();
    let mut ans = s.len() as i32;
    for i in idx..s.len() {
        if is_palindrome(s, idx, i) {
            ans = ans.min(recur(s, memo, i + 1) + 1)
        }
    }

    memo[idx] = ans;

    ans
}

fn is_palindrome(s: &str, mut ss: usize, mut se: usize) -> bool {
    let sbytes = s.as_bytes();
    while ss < se {
        if sbytes[ss] != sbytes[se] {
            return false;
        }
        ss += 1;
        se -= 1;
    }
    true
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_cut() {
        assert_eq!(is_palindrome("aabaa", 0, 2), false);
        assert_eq!(Solution::min_cut(String::from("aab")), 1);
    }
}
