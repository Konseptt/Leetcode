use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;



impl Solution {
    pub fn tree_queries(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<i32> {
        let mut d = HashMap::new();
        
        fn f(node: Option<Rc<RefCell<TreeNode>>>, d: &mut HashMap<i32, i32>) -> i32 {
            if let Some(n) = node {
                let n = n.borrow();
                let l = f(n.left.clone(), d);
                let r = f(n.right.clone(), d);
                let depth = 1 + l.max(r);
                d.insert(n.val, depth);
                depth
            } else {
                0
            }
        }
        
        f(root.clone(), &mut d);
        
        let mut res = vec![0; d.len() + 1];
        
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, depth: i32, rest: i32, d: &HashMap<i32, i32>, res: &mut Vec<i32>) {
            if let Some(n) = node {
                let n = n.borrow();
                let depth = depth + 1;
                res[n.val as usize] = rest;
                dfs(n.left.clone(), depth, rest.max(depth + d.get(&n.right.as_ref().map_or(0, |r| r.borrow().val)).unwrap_or(&0)), d, res);
                dfs(n.right.clone(), depth, rest.max(depth + d.get(&n.left.as_ref().map_or(0, |l| l.borrow().val)).unwrap_or(&0)), d, res);
            }
        }
        
        dfs(root, -1, 0, &d, &mut res);
        
        queries.into_iter().map(|v| res[v as usize]).collect()
    }
}