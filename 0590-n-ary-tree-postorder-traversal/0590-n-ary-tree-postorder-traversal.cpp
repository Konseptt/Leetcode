class Solution {
public:
    void postorderTraversal(Node* node, std::vector<int>& res) {
        if (!node) {
            return;
        }

        for (auto n : node->children) {
            postorderTraversal(n, res);
        }

        res.push_back(node->val);
    }

    vector<int> postorder(Node* root) {
        std::vector<int> res;
        postorderTraversal(root, res);
        return res;
    }
};