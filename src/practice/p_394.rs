impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut ans = String::new();
        let mut idx = 0;
        while idx < s.len() {
            idx = recur(s.as_bytes(), idx, &mut ans);
        }
        ans
    }
}

fn recur(s: &[u8], mut idx: usize, cur: &mut String) -> usize {
    if idx >= s.len() {
        return s.len();
    }

    while idx < s.len() && s[idx] != b']' {
        // parse num
        let num = if s[idx] > b'9' {
            1
        } else {
            let mut n = 0i32;
            while idx < s.len() && s[idx] != b'[' {
                n = (n * 10) + s[idx] as i32 - b'0' as i32;
                idx += 1;
            }
            n
        };

        if s[idx] == b'[' {
            let mut subcur = String::new();
            idx = recur(s, idx + 1, &mut subcur);
            for _ in 0..num {
                cur.push_str(&subcur);
            }
        } else {
            cur.push(s[idx] as char);
            idx += 1;
        }
    }
    idx + 1
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_four_key_keyboard() {
        assert_eq!(
            Solution::decode_string("2[f2[e]g]i".to_string()),
            "feegfeegi"
        );
        // assert_eq!(Solution::decode_string("3[a2[c]]".to_string()), "accaccacc");
    }
}
