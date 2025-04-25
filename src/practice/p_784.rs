impl Solution {
    pub fn letter_case_permutation(s: String) -> Vec<String> {
        let mut ans = vec![];
        backtrack(s.as_bytes(), &mut String::new(), &mut ans);
        ans
    }
}

fn backtrack(letters: &[u8], cur: &mut String, ans: &mut Vec<String>) {
    let idx = cur.len();
    if idx == letters.len() {
        ans.push(cur.clone());
        return;
    }
    if letters[idx] >= b'0' && letters[idx] <= b'9' {
        cur.push(letters[idx] as char);
        backtrack(letters, cur, ans);
    } else {
        cur.push(letters[idx] as char);
        backtrack(letters, cur, ans);
        cur.pop();
        cur.push((letters[idx] - 32) as char);
        backtrack(letters, cur, ans);
    }
    cur.pop();
}

struct Solution;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_letter_case_permutation() {
        println!(
            "{:?}",
            Solution::letter_case_permutation("a1b2".to_string())
        )
    }
}
