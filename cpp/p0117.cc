#include "x.h"

class Solution {
public:
  Node *connect(Node *root) {
    if (!root) {
      return nullptr;
    }
    auto p = root;
    Node *last = nullptr, *start = nullptr;
    while (p) {
      for (auto x = p; x; x = x->next) {
        if (x->left) {
          if (!start) {
            start = x->left;
          }
          if (last) {
            last->next = x->left;
          }
          last = x->left;
        }
        if (x->right) {
          if (!start) {
            start = x->right;
          }
          if (last) {
            last->next = x->right;
          }
          last = x->right;
        }
      }
      last = nullptr;
      p = start;
      start = nullptr;
    }
    return root;
  }
};
