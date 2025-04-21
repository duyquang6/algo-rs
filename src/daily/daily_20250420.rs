impl Solution {
    pub fn num_rabbits(answers: Vec<i32>) -> i32 {
        let n = answers.len();
        let mut fq = vec![0; 1001];
        let mut ans = 0;

        for &n in &answers {
            fq[n as usize] += 1;
        }

        for (num_same_color, &n) in fq.iter().enumerate() {
            let group_size = 1 + num_same_color as i32;
            ans += ((n + group_size - 1) / group_size) * group_size;
        }

        ans
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_rabbits() {
        assert_eq!(Solution::num_rabbits(vec![1, 1, 2],), 5)
    }
}
