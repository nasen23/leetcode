#include "x.h"

class Node {
public:
  int val;
  Node *next;
  Node *random;

  Node(int val, Node *next = nullptr, Node *random = nullptr)
      : val(val), next(next), random(random) {}
};

class Solution {
public:
  Node *copyRandomList(Node *head) {
    if (!head)
      return nullptr;
    Node *p = head;
    while (p) {
      Node *c = new Node(p->val, p->next, nullptr);
      Node *t = p->next;
      p->next = c;
      p = t;
    }
    p = head;
    while (p) {
      if (p->random)
        p->next->random = p->random->next;
      p = p->next->next;
    }
    p = head;
    Node *res = p->next;
    while (p->next) {
      Node *c = p->next;
      p->next = p->next->next;
      p = c;
    }
    return res;
  }
};
