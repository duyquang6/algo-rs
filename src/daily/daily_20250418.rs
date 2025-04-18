impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut ans: String = String::from("1");

        for _ in 0..n - 1 {
            let mut prev = ans.as_bytes()[0] as char;
            let mut cnt = 1;
            let mut temp = String::new();
            for c in (&ans).chars().skip(1) {
                if c == prev {
                    cnt += 1;
                } else {
                    temp.push_str(&format!("{}", cnt));
                    temp.push(prev);
                    prev = c;
                    cnt = 1;
                }
            }
            temp.push_str(&format!("{}", cnt));
            temp.push(prev);

            ans = temp;
        }

        ans
    }
}

struct Solution;

mod tests {
    use super::*;

    #[test]
    fn test_count_and_say() {
        assert_eq!(Solution::count_and_say(4), "1211")
    }
}
