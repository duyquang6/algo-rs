use std::collections::HashMap;

struct Solution;

impl Solution {
    fn minimum_palindrome_remove(arr: Vec<i32>) -> i32 {
        recur(&arr, &mut HashMap::new(), 0, arr.len() - 1)
    }
}

fn recur(arr: &Vec<i32>, memo: &mut HashMap<usize, i32>, start: usize, end: usize) -> i32 {
    if start >= end {
        return 1;
    }

    let key = get_key(start, end);
    if let Some(&v) = memo.get(&key) {
        return v;
    }

    let mut ans = arr.len() as i32;
    if arr[start] == arr[end] {
        ans = ans.min(recur(arr, memo, start + 1, end - 1));
    }
    for k in start..end {
        ans = ans.min(recur(arr, memo, start, k) + recur(arr, memo, k + 1, end))
    }

    memo.insert(key, ans);
    ans
}

fn get_key(start: usize, end: usize) -> usize {
    start * 1000 + end
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_minimum_palindrome_remove() {
        assert_eq!(Solution::minimum_palindrome_remove(vec![1, 3, 4, 1, 5]), 3);
    }
}
