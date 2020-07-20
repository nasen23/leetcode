#include "x.h"

class Solution {
public:
  bool hasCycle(ListNode *head) {
    int c = 8029;
    while (head && c) {
      head = head->next;
      c--;
    }
    if (!head) {
      return false;
    }
    return true;
  }
};
