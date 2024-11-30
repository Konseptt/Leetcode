use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn valid_arrangement(pairs: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut graph: HashMap<i32, VecDeque<i32>> = HashMap::new();
        let mut out_degree: HashMap<i32, i32> = HashMap::new();
        let mut in_degree: HashMap<i32, i32> = HashMap::new();

        for pair in &pairs {
            let start = pair[0];
            let end = pair[1];
            
            graph.entry(start)
                .or_insert_with(VecDeque::new)
                .push_back(end);
            
            *out_degree.entry(start).or_insert(0) += 1;
            *in_degree.entry(end).or_insert(0) += 1;
        }

        let start_node = Self::get_start_node(&graph, &out_degree, &in_degree, &pairs);

        let mut ans = Vec::new();
        Self::euler(&mut graph, start_node, &mut ans);
        ans.reverse();
        ans
    }

    fn get_start_node(
        graph: &HashMap<i32, VecDeque<i32>>, 
        out_degree: &HashMap<i32, i32>,
        in_degree: &HashMap<i32, i32>, 
        pairs: &Vec<Vec<i32>>
    ) -> i32 {
        for &u in graph.keys() {
            if out_degree.get(&u).cloned().unwrap_or(0) - in_degree.get(&u).cloned().unwrap_or(0) == 1 {
                return u;
            }
        }
        pairs[0][0]
    }

    fn euler(
        graph: &mut HashMap<i32, VecDeque<i32>>, 
        u: i32, 
        ans: &mut Vec<Vec<i32>>
    ) {
        while let Some(v) = graph.get_mut(&u).and_then(|stack| stack.pop_front()) {
            Self::euler(graph, v, ans);
            ans.push(vec![u, v]);
        }
    }
}