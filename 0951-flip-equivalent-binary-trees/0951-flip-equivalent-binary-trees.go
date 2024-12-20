

func flipEquiv(root1 *TreeNode, root2 *TreeNode) bool {
    var dfs func(root1, root2 *TreeNode) bool
    dfs = func(root1, root2 *TreeNode) bool {
        if root1 == root2 || (root1 == nil && root2 == nil) {
            return true
        }
        if root1 == nil || root2 == nil || root1.Val != root2.Val {
            return false
        }
        return (dfs(root1.Left, root2.Left) && dfs(root1.Right, root2.Right)) || (dfs(root1.Left, root2.Right) && dfs(root1.Right, root2.Left))
    }
    return dfs(root1, root2)
}