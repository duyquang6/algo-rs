use std::{cmp::Reverse, collections::*};
impl Solution {
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut max_heap = BinaryHeap::new();
        for &val in matrix.iter().flatten() {
            if max_heap.len() < k as usize {
                max_heap.push(val);
                continue;
            }
            if let Some(&top) = max_heap.peek() {
                if val >= top {
                    continue;
                }
                max_heap.pop();
                max_heap.push(val);
            }
        }
        max_heap.pop().unwrap()
    }
}
struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kth_smallest() {
        assert_eq!(
            Solution::kth_smallest(
                vec![
                    [1, 5, 9].to_vec(),
                    [10, 11, 13].to_vec(),
                    [12, 13, 15].to_vec(),
                ],
                8,
            ),
            13
        );
    }
}
