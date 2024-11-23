impl Solution {
    pub fn rotate_the_box(r#box: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let m = r#box.len();
        let n = r#box[0].len();
        
        let mut ans = vec![vec!['.'; m]; n];
        
        for i in 0..m {
            for j in 0..n {
                ans[j][m - i - 1] = r#box[i][j];
            }
        }
        
        for j in 0..m {
            let mut q = Vec::new();
            for i in (0..n).rev() {
                if ans[i][j] == '*' {
                    q.clear();
                } else if ans[i][j] == '.' {
                    q.push(i);
                } else if !q.is_empty() {
                    ans[q[0]][j] = '#';
                    ans[i][j] = '.';
                    q.remove(0);
                    q.push(i);
                }
            }
        }
        
        ans
    }
}
