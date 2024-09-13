use std::collections::{HashMap, HashSet, VecDeque};

impl Solution {
    pub fn max_moves(kx: i32, ky: i32, positions_input: Vec<Vec<i32>>) -> i32 {
        // Collect all positions, including the initial knight position
        let mut positions: Vec<(i32, i32)> = Vec::new();
        positions.push((kx, ky));
        for pos in &positions_input {
            positions.push((pos[0], pos[1]));
        }

        // Map positions to indices
        let n_positions = positions.len();
        let mut index_map: HashMap<(i32, i32), usize> = HashMap::new();
        for (idx, &pos) in positions.iter().enumerate() {
            index_map.insert(pos, idx);
        }

        // Precompute cost_matrix
        let mut cost_matrix = vec![vec![-1; n_positions]; n_positions];
        for i in 0..n_positions {
            let distances = Self::knight_min_moves(positions[i].0, positions[i].1, &positions);
            for j in 0..n_positions {
                let pos_j = positions[j];
                if let Some(&cost) = distances.get(&pos_j) {
                    cost_matrix[i][j] = cost;
                }
            }
        }

        // Initialize DP memoization
        let n_pawns = n_positions - 1;
        let full_S = (1 << n_pawns) - 1;
        let mut memo = HashMap::new();

        // Start the DP with the initial knight position, all pawns, and Alice's turn
        Self::dp(0, full_S, 0, &cost_matrix, &mut memo)
    }

    // Function to compute minimal knight moves from (sx, sy) to target positions
    fn knight_min_moves(
        sx: i32,
        sy: i32,
        target_positions: &Vec<(i32, i32)>,
    ) -> HashMap<(i32, i32), i32> {
        let mut dist = vec![vec![-1; 50]; 50];
        let mut queue = VecDeque::new();
        queue.push_back((sx, sy));
        dist[sx as usize][sy as usize] = 0;

        let directions = [
            (-2, -1),
            (-1, -2),
            (1, -2),
            (2, -1),
            (2, 1),
            (1, 2),
            (-1, 2),
            (-2, 1),
        ];

        let mut result = HashMap::new();
        let mut positions_set: HashSet<(i32, i32)> = target_positions.iter().cloned().collect();

        if positions_set.contains(&(sx, sy)) {
            result.insert((sx, sy), 0);
            positions_set.remove(&(sx, sy));
        }

        while let Some((x, y)) = queue.pop_front() {
            for &(dx, dy) in directions.iter() {
                let nx = x + dx;
                let ny = y + dy;
                if nx >= 0
                    && nx < 50
                    && ny >= 0
                    && ny < 50
                    && dist[nx as usize][ny as usize] == -1
                {
                    dist[nx as usize][ny as usize] = dist[x as usize][y as usize] + 1;
                    queue.push_back((nx, ny));

                    if positions_set.contains(&(nx, ny)) {
                        result.insert((nx, ny), dist[nx as usize][ny as usize]);
                        positions_set.remove(&(nx, ny));
                        if positions_set.is_empty() {
                            return result;
                        }
                    }
                }
            }
        }

        result
    }

    // DP function with memoization
    fn dp(
        k: usize,
        S: usize,
        player: usize,
        cost_matrix: &Vec<Vec<i32>>,
        memo: &mut HashMap<(usize, usize, usize), i32>,
    ) -> i32 {
        let key = (k, S, player);
        if let Some(&res) = memo.get(&key) {
            return res;
        }
        if S == 0 {
            return 0;
        }
        let n_positions = cost_matrix.len();
        let mut result: i32;
        let mut move_made = false;
        if player == 0 {
            // Alice's turn (maximize total moves)
            result = 0;
            for i in 1..n_positions {
                let bit = 1 << (i - 1);
                if S & bit != 0 {
                    let cost = cost_matrix[k][i];
                    if cost == -1 {
                        continue;
                    }
                    move_made = true;
                    let total_cost = cost + Self::dp(i, S ^ bit, 1, cost_matrix, memo);
                    if total_cost > result {
                        result = total_cost;
                    }
                }
            }
            if !move_made {
                result = 0;
            }
        } else {
            // Bob's turn (minimize total moves)
            result = std::i32::MAX;
            for i in 1..n_positions {
                let bit = 1 << (i - 1);
                if S & bit != 0 {
                    let cost = cost_matrix[k][i];
                    if cost == -1 {
                        continue;
                    }
                    move_made = true;
                    let total_cost = cost + Self::dp(i, S ^ bit, 0, cost_matrix, memo);
                    if total_cost < result {
                        result = total_cost;
                    }
                }
            }
            if !move_made {
                result = 0;
            }
        }
        memo.insert(key, result);
        result
    }
}
