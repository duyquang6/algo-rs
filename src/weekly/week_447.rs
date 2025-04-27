use std::collections::*;

impl Solution {
    pub fn count_covered_buildings(n: i32, buildings: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;

        let mut row_min_max = HashMap::new();
        let mut col_min_max = HashMap::new();

        for b in &buildings {
            let (x, y) = (b[0], b[1]);

            let item = row_min_max.entry(x).or_insert([y, y]);
            *item = [item[0].min(y), item[1].max(y)];

            let item = col_min_max.entry(y).or_insert([x, x]);
            *item = [item[0].min(x), item[1].max(x)];
        }

        for b in buildings {
            let (x, y) = (b[0], b[1]);
            let &[x_min, x_max] = col_min_max.get(&y).unwrap();
            let &[y_min, y_max] = row_min_max.get(&x).unwrap();
            if x_min < x && x < x_max && y_min < y && y < y_max {
                ans += 1;
            }
        }

        ans
    }

    pub fn path_existence_queries_I(
        n: i32,
        nums: Vec<i32>,
        max_diff: i32,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        // union-find to determine connected nodes
        let mut groups: Vec<i32> = (0..n).collect();
        // let mut sizes: Vec<i32> = vec![1; n as usize];

        fn find(groups: &Vec<i32>, mut x: i32) -> i32 {
            while x != groups[x as usize] {
                x = groups[x as usize]
            }
            x
        }

        fn union(groups: &mut Vec<i32>, x: i32, y: i32) {
            let (rx, ry) = (find(groups, x), find(groups, y));
            if rx == ry {
                return;
            }

            // merge rx to ry
            groups[rx as usize] = ry;
        }

        for i in 0..nums.len() - 1 {
            if nums[i + 1] - nums[i] <= max_diff {
                union(&mut groups, i as i32 + 1, i as i32);
            }
        }

        let mut ans = vec![];

        for q in queries {
            ans.push(if find(&groups, q[0]) == find(&groups, q[1]) {
                true
            } else {
                false
            })
        }

        ans
    }

    pub fn concatenated_divisibility(mut nums: Vec<i32>, k: i32) -> Vec<i32> {
        nums.sort_unstable();

        let mut ans = Vec::new();
        dfs(&nums, &mut HashMap::new(), 0, 0, k, &mut ans);

        ans
    }

    // pub fn path_existence_queries(
    //     n: i32,
    //     nums: Vec<i32>,
    //     max_diff: i32,
    //     queries: Vec<Vec<i32>>,
    // ) -> Vec<i32> {
    //     vec![]
    // }
}

fn get_num_pow(mut n: i32) -> i32 {
    let mut ans = 1;
    while n > 0 {
        n /= 10;
        ans *= 10;
    }
    ans
}

fn dfs(
    nums: &Vec<i32>,
    memo: &mut HashMap<(i32, i32), bool>,
    bitmask: i32,
    remainder: i32,
    k: i32,
    path: &mut Vec<i32>,
) -> bool {
    if bitmask == (1 << nums.len()) - 1 {
        return remainder == 0;
    }

    let key = (bitmask, remainder);
    if let Some(&res) = memo.get(&key) {
        return res;
    }

    for i in 0..nums.len() {
        if bitmask & (1 << i) > 0 {
            continue;
        }
        let pow = get_num_pow(nums[i]);
        path.push(nums[i]);
        let is_exist = dfs(
            nums,
            memo,
            bitmask | (1 << i),
            ((remainder * pow) % k + nums[i]) % k,
            k,
            path,
        );
        if is_exist {
            memo.insert(key, true);
            return true;
        }
        path.pop();
    }

    memo.insert(key, false);
    false
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_concatenated_divisibility() {
        println!("{:?}", Solution::concatenated_divisibility(vec![2, 7], 4));
    }
}
