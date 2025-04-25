impl Solution {
    pub fn maximal_network_rank(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let mut graph = vec![vec![0; n as usize]; n as usize];
        for edge in roads {
            graph[edge[0] as usize][edge[1] as usize] = 1;
            graph[edge[1] as usize][edge[0] as usize] = 1;
        }
        let direct_road_count: Vec<i32> = graph.iter().map(|row| row.iter().sum()).collect();

        let mut best = 0;
        for i in 0..n as usize {
            for j in i + 1..n as usize {
                let mut ans = direct_road_count[i] + direct_road_count[j];
                best = best.max(if graph[i][j] == 1 { ans - 1 } else { ans });
            }
        }

        best
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximal_network_rank() {
        assert_eq!(
            Solution::maximal_network_rank(
                4,
                vec![
                    [0, 1].to_vec(),
                    [0, 3].to_vec(),
                    [1, 2].to_vec(),
                    [1, 3].to_vec()
                ]
            ),
            4
        )
    }
}
