struct Solution;

impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let (m, n) = (board.len() as i32, board[0].len() as i32);
        let mut root = Trie::new();
        let mut visited = vec![vec![false; n as usize]; m as usize];

        for word in words {
            insert(&word, &mut root, 0);
        }

        dfs(&board, &mut visited, &mut root, m, n);

        let mut ans = Vec::new();
        dfs_trie(&root, &mut String::new(), &mut ans);
        ans
    }
}

fn dfs_trie(root: &Trie, cur: &mut String, ans: &mut Vec<String>) {
    if root.is_ans {
        ans.push(cur.clone());
    }
    for (idx, child) in root.children.iter().enumerate() {
        if child.is_none() {
            continue;
        }

        let child_node = child.as_deref().unwrap();
        cur.push((idx as u8 + 97) as char);
        dfs_trie(child_node, cur, ans);
        cur.pop();
    }
}

fn insert(word: &str, root: &mut Trie, idx: usize) {
    if idx == word.len() {
        root.is_end = true;
        return;
    }
    let c = (word.as_bytes()[idx] - 97) as usize;
    root.children[c].get_or_insert_with(|| Box::new(Trie::new()));
    return insert(word, root.children[c].as_mut().unwrap(), idx + 1);
}

fn search(word: &str, root: &Trie, idx: usize) -> bool {
    if idx == word.len() {
        return true;
    }

    let c = (word.as_bytes()[idx] - 97) as usize;
    if root.children[c].is_none() {
        return false;
    }

    return search(word, root.children[c].as_ref().unwrap(), idx + 1);
}

const DIRS: [[i32; 2]; 4] = [[0, 1], [0, -1], [-1, 0], [1, 0]];

fn dfs(board: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, node: &mut Trie, i: i32, j: i32) {
    let (m, n) = (board.len() as i32, board[0].len() as i32);
    if i < 0 || (i >= m && j != n) {
        return;
    }

    if j < 0 || (j >= n && i != m) {
        return;
    }

    if i == m && j == n {
        // init point
        for x in 0..m {
            for y in 0..n {
                dfs(board, visited, node, x, y)
            }
        }
        return;
    }
    if visited[i as usize][j as usize] {
        return;
    }
    visited[i as usize][j as usize] = true;

    let c = board[i as usize][j as usize] as usize - 97;
    if node.children[c].is_none() {
        visited[i as usize][j as usize] = false;
        return;
    }

    let node = node.children[c].as_mut().unwrap().as_mut();
    if node.is_end {
        node.is_ans = true;
    }

    for d in DIRS {
        let (dx, dy) = (d[0], d[1]);
        let (nx, ny) = (dx + i, dy + j);
        if nx >= 0 && nx < m && ny >= 0 && ny < n {
            dfs(board, visited, node, nx, ny)
        }
    }

    visited[i as usize][j as usize] = false;
}

#[derive(Debug)]
struct Trie {
    children: [Option<Box<Trie>>; 26],
    is_end: bool,
    is_ans: bool,
}

impl Trie {
    fn new() -> Self {
        Trie {
            children: Default::default(),
            is_end: false,
            is_ans: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_words() {
        assert_eq!(
            Solution::find_words(
                vec![
                    vec!['o', 'a', 'a', 'n'],
                    vec!['e', 't', 'a', 'e'],
                    vec!['i', 'h', 'k', 'r'],
                    vec!['i', 'f', 'l', 'v']
                ],
                ["oath", "pea", "eat", "rain"]
                    .iter()
                    .map(|&x| String::from(x))
                    .collect(),
            ),
            ["eat", "oath"]
        );
    }
}
