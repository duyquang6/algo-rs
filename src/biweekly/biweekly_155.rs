use std::collections::*;
const MOD: i64 = 1e9 as i64 + 7;

impl Solution {
    pub fn find_common_response(responses: Vec<Vec<String>>) -> String {
        let mut fq = HashMap::new();
        let mut best = &responses[0][0];
        let mut max_cnt = 0;
        for response in &responses {
            let mut set = HashSet::new();
            for r in response {
                set.insert(r);
            }
            for item in set {
                let cnt = fq.entry(item).and_modify(|x| *x += 1).or_insert(1);

                if *cnt > max_cnt {
                    max_cnt = *cnt;
                    best = item;
                } else if *cnt == max_cnt {
                    best = best.min(item)
                }
            }
        }

        best.to_string()
    }

    pub fn base_unit_conversions(conversions: Vec<Vec<i32>>) -> Vec<i32> {
        let n = conversions.len() + 1;
        let mut ans = vec![1; n];
        let mut q = VecDeque::new();
        let mut visited = vec![false; n];

        // create graph
        let mut graph = vec![vec![]; n];
        for edge in &conversions {
            let (source, target, qty) = (edge[0], edge[1], edge[2]);
            graph[source as usize].push([target, qty]);
        }

        q.push_back([0i32, 1]);
        while q.len() > 0 {
            let [node, factor] = q.pop_front().unwrap();
            if visited[node as usize] {
                continue;
            }
            visited[node as usize] = true;
            ans[node as usize] = factor;

            for &[next, delta_factor] in &graph[node as usize] {
                if visited[next as usize] {
                    continue;
                }
                q.push_back([next, ((factor as i64 * delta_factor as i64) % MOD) as i32]);
            }
        }

        ans
    }

    pub fn count_cells(grid: Vec<Vec<char>>, pattern: String) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        const MOD: i64 = i32::MAX as i64;

        // rabin karp to detect pattern
        let mut target = 0i64;
        for &c in pattern.as_bytes() {
            let c = c as i64 - 97;
            target = ((target * 26) % MOD + c) % MOD;
        }
        let mut pow = 1;
        for _ in 0..pattern.len() {
            pow = (pow * 26) % MOD;
        }
        let mut grid_pattern = vec![0; n * m];

        let mut last = 0;
        let mut cur = 0;
        for i in 0..m * n {
            let (r, c) = (i / n, i % n);
            let c = grid[r][c] as i64 - 97;
            if i < pattern.len() {
                cur = (cur * 26 + c) % MOD;
            } else {
                let pi = i - pattern.len();
                let (pr, pc) = (pi / n, pi % n);
                let pc = grid[pr][pc] as i64 - 97;

                cur = (cur * 26 - (pow * pc) % MOD + MOD + c) % MOD;
            }
            if i >= pattern.len() - 1 && cur == target {
                let start = (i as i32 - pattern.len() as i32 + 1).max(last as i32) as usize;
                for j in start..=i {
                    grid_pattern[j] = 1;
                }
                last = i;
            }
        }

        let mut last = 0;
        let mut cur = 0;
        for i in 0..m * n {
            let (r, c) = (i % m, i / m);
            let c = grid[r][c] as i64 - 97;
            if i < pattern.len() {
                cur = (cur * 26 + c) % MOD;
            } else {
                let pi = i - pattern.len();
                let (pr, pc) = (pi % m, pi / m);
                let pc = grid[pr][pc] as i64 - 97;

                cur = (cur * 26 - (pow * pc) % MOD + MOD + c) % MOD;
            }
            if i >= pattern.len() - 1 && cur == target {
                let start = (i as i32 - pattern.len() as i32 + 1).max(last as i32) as usize;
                for j in start..=i {
                    let (r, c) = (j % m, j / m);
                    let j = r * n + c;
                    grid_pattern[j] |= 2;
                }
                last = i;
            }
        }

        // check if row and col is matching
        grid_pattern
            .iter()
            .map(|&x| if x == 3 { 1 } else { 0 })
            .sum()
    }
}
struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_cells() {
        // Solution::count_cells(
        //     vec![
        //         vec!['a', 'a', 'c', 'c'],
        //         vec!['b', 'b', 'b', 'c'],
        //         vec!['a', 'a', 'b', 'a'],
        //         vec!['c', 'a', 'a', 'c'],
        //         vec!['a', 'a', 'c', 'c'],
        //     ],
        //     "abaca".to_string(),
        // );

        // println!(
        //     "{}",
        //     Solution::count_cells(
        //         vec![vec!['k', 'k', 'k'], vec!['k', 'k', 'k']],
        //         "kk".to_string(),
        //     )
        // );

        println!(
            "{}",
            Solution::count_cells(
                vec![
                    vec!['b', 'a', 'b'],
                    vec!['a', 'b', 'a'],
                    vec!['b', 'c', 'b']
                ],
                "ababa".to_string(),
            )
        );
    }
}
