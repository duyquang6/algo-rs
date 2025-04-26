struct Solution;

impl Solution {
    fn longestKSubstr(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let mut fq: Vec<i32> = vec![0; 26];
        let mut left = 0;
        let mut best = 0;
        for right in 0..s.len() {
            fq[s[right] as usize - 97] += 1;
            while left < right && fq.iter().map(|&x| if x > 0 { 1 } else { 0 }).sum::<i32>() > k {
                fq[s[left] as usize - 97] -= 1;
                left += 1;
            }
            best = best.max(right - left + 1);
        }

        best as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longestKSubstr() {
        assert_eq!(Solution::longestKSubstr("eceba".to_string(), 2), 3);
        assert_eq!(Solution::longestKSubstr("aa".to_string(), 1), 2);
    }
}
