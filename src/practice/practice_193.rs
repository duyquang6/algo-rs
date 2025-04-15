use std::io::Read;

impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let mut ans = Vec::new();
        recur(s.as_str(), 0, &mut String::new(), 0, &mut ans);
        ans
    }
}

fn check_group_valid(data: &str) -> bool {
    if data.len() > 3 {
        return false;
    }
    if data.len() == 0 {
        return false;
    }

    let first = data.as_bytes()[0];
    if data.len() == 1 && first == b'0' {
        return true;
    }

    if first == b'0' {
        return false;
    }

    if data.len() == 3 && data > "255" {
        return false;
    }

    true
}

fn recur(s: &str, idx: usize, cur: &mut String, dot_count: i32, ans: &mut Vec<String>) {
    if dot_count == 3 {
        let group = &s[idx..];
        if check_group_valid(group) {
            cur.push_str(group);
            ans.push(cur.clone());
            for _ in group.chars() {
                cur.pop();
            }
        }
        return;
    }

    for se in idx..s.len().min(idx + 3) {
        let c = s.as_bytes()[se] as char;
        cur.push(c);
        if check_group_valid(&s[idx..=se]) {
            cur.push('.');
            recur(s, se + 1, cur, dot_count + 1, ans);
            cur.pop();
        }
    }
    for _ in idx..s.len().min(idx + 3) {
        cur.pop();
    }
}
struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_restore_ip_addresses() {
        assert_eq!(
            Solution::restore_ip_addresses(String::from("25525511135")),
            vec!["255.255.11.135", "255.255.111.35"]
        );
        assert_eq!(
            Solution::restore_ip_addresses(String::from("0000")),
            vec!["0.0.0.0"]
        );
        assert_eq!(
            Solution::restore_ip_addresses(String::from("1111")),
            vec!["1.1.1.1"]
        );
        assert_eq!(
            Solution::restore_ip_addresses(String::from("101023")),
            vec![
                "1.0.10.23",
                "1.0.102.3",
                "10.1.0.23",
                "10.10.2.3",
                "101.0.2.3"
            ]
        );
    }
}
