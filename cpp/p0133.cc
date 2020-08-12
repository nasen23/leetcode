#include "x.h"

class Node {
public:
  int val;
  vector<Node *> neighbors;

  Node() {
    val = 0;
    neighbors = vector<Node *>();
  }

  Node(int _val) {
    val = _val;
    neighbors = vector<Node *>();
  }

  Node(int _val, vector<Node *> _neighbors) {
    val = _val;
    neighbors = _neighbors;
  }
};

class Solution {
private:
  Node *nodes[101] = {nullptr};

public:
  Node *cloneGraph(Node *node) {
    if (!node) {
      return nullptr;
    }
    if (nodes[node->val]) {
      return nodes[node->val];
    }
    nodes[node->val] = new Node(node->val);
    for (auto neighbor : node->neighbors) {
      auto cloned = cloneGraph(neighbor);
      nodes[node->val]->neighbors.push_back(cloned);
    }
    return nodes[node->val];
  }
};
