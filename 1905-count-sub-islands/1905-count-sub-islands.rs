impl Solution {
    pub fn count_sub_islands(mut grid1: Vec<Vec<i32>>, mut grid2: Vec<Vec<i32>>) -> i32 {
        const DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        let m = grid1.len();
        let n = grid1[0].len();
        
        fn dfs(grid1: &Vec<Vec<i32>>, grid2: &mut Vec<Vec<i32>>, i: usize, j: usize, m: usize, n: usize) -> bool {
            if i >= m || j >= n || grid2[i][j] == 0 {
                return true;
            }
            grid2[i][j] = 0;
            let mut is_sub_island = grid1[i][j] == 1;
            for &(dx, dy) in &DIRS {
                let x = i as i32 + dx;
                let y = j as i32 + dy;
                if x >= 0 && x < m as i32 && y >= 0 && y < n as i32 {
                    is_sub_island &= dfs(grid1, grid2, x as usize, y as usize, m, n);
                }
            }
            is_sub_island
        }
        
        let mut ans = 0;
        for i in 0..m {
            for j in 0..n {
                if grid2[i][j] == 1 && dfs(&grid1, &mut grid2, i, j, m, n) {
                    ans += 1;
                }
            }
        }
        ans
    }
}