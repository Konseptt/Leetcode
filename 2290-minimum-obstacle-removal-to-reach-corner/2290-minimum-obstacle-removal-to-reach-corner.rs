impl Solution {
    pub fn minimum_obstacles(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        
        let mut queue = std::collections::VecDeque::new();
        
        let mut dist = vec![vec![i32::MAX; n]; m];
        
        let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        
        queue.push_back((0, 0, 0));
        dist[0][0] = 0;
        
        while let Some((x, y, obstacles)) = queue.pop_front() {
            if x == m - 1 && y == n - 1 {
                return obstacles;
            }
            
            for (dx, dy) in dirs.iter() {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                
                if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 {
                    let (nx, ny) = (nx as usize, ny as usize);
                    let new_obstacles = obstacles + grid[nx][ny];
                    
                    if new_obstacles < dist[nx][ny] {
                        dist[nx][ny] = new_obstacles;
                        
                        if grid[nx][ny] == 0 {
                            queue.push_front((nx, ny, new_obstacles));
                        } else {
                            queue.push_back((nx, ny, new_obstacles));
                        }
                    }
                }
            }
        }
        
        -1
    }
}