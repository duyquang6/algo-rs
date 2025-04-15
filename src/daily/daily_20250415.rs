use std::collections::HashMap;

fn update(tree: &mut Vec<i32>, node: i32, ss: i32, se: i32, idx: i32, value: i32) {
    if idx < ss || idx > se {
        return;
    }

    if ss == se {
        tree[node as usize] = value;
        return;
    }

    let mid = (ss + se) / 2;
    update(tree, node * 2 + 1, ss, mid, idx, value);
    update(tree, node * 2 + 2, mid + 1, se, idx, value);

    tree[node as usize] = tree[(node * 2 + 1) as usize] + tree[(node * 2 + 2) as usize]
}

pub fn query(tree: &Vec<i32>, node: i32, ss: i32, se: i32, qs: i32, qe: i32) -> i32 {
    if qe < qs {
        return 0;
    }
    if qe < ss || qs > se {
        return 0;
    }

    if ss >= qs && se <= qe {
        return tree[node as usize];
    }

    let mid = (ss + se) / 2;

    let left = query(tree, node * 2 + 1, ss, mid, qs, qe);
    let right = query(tree, node * 2 + 2, mid + 1, se, qs, qe);

    return left + right;
}

impl Solution {
    pub fn good_triplets(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        let n = nums1.len();
        let mut nums = vec![0; n];
        let mut map = HashMap::new();

        for (i, &n) in nums2.iter().enumerate() {
            map.insert(n, i);
        }

        for (i, &n) in nums1.iter().enumerate() {
            nums[i] = *map.get(&n).unwrap() as i32
        }

        let mut left_tree = vec![0; 4 * n];
        let mut right_tree = vec![0; 4 * n];

        update(&mut left_tree, 0, 0, (n - 1) as i32, nums[0], 1);
        for i in 2..n {
            update(&mut right_tree, 0, 0, (n - 1) as i32, nums[i], 1);
        }

        let mut ans = 0i64;

        for i in 1..n - 1 {
            let left = query(&left_tree, 0, 0, (n - 1) as i32, 0, nums[i] - 1);
            let right = query(
                &right_tree,
                0,
                0,
                (n - 1) as i32,
                nums[i] + 1,
                (n - 1) as i32,
            );
            ans += left as i64 * right as i64;

            update(&mut left_tree, 0, 0, (n - 1) as i32, nums[i], 1);
            update(&mut right_tree, 0, 0, (n - 1) as i32, nums[i], 0);
        }

        return ans;
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_good_triplets() {
        assert_eq!(
            Solution::good_triplets(vec![2, 0, 1, 3], vec![0, 1, 2, 3],),
            1
        );
        assert_eq!(
            Solution::good_triplets(vec![4, 0, 1, 3, 2], vec![4, 1, 0, 2, 3],),
            4
        );
    }
}
