use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn minimum_obstacles(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        
        let mut dist = vec![vec![i32::MAX; n]; m];
        let mut heap = BinaryHeap::new();
        
        heap.push(Reverse((0, 0, 0)));
        dist[0][0] = 0;
        
        let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        
        while let Some(Reverse((obstacles, x, y))) = heap.pop() {
            if x == m - 1 && y == n - 1 {
                return obstacles;
            }
            
            if obstacles > dist[x][y] {
                continue;
            }
            
            for (dx, dy) in dirs {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                
                if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 {
                    let (nx, ny) = (nx as usize, ny as usize);
                    let new_obstacles = obstacles + grid[nx][ny];
                    
                    if new_obstacles < dist[nx][ny] {
                        dist[nx][ny] = new_obstacles;
                        heap.push(Reverse((new_obstacles, nx, ny)));
                    }
                }
            }
        }
        
        -1
    }
}