impl Solution {
    pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
        if (m * n) as usize != original.len() {
            return vec![];
        }
        let mut ans = vec![vec![0; n as usize]; m as usize];
        for i in 0..m as usize {
            for j in 0..n as usize {
                ans[i][j] = original[i * n as usize + j];
            }
        }
        ans
    }
}