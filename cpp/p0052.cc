#include "x.h"

class Solution {
  int mask;

public:
  int totalNQueens(int n) {
    mask = (1 << n) - 1;
    int res = 0;
    function<void(int, int, int, int)> dfs = [&](int row, int l, int r, int u) {
      if (row == n) {
        res++;
      } else {
        int bitmap = mask & ~(l | r | u);
        while (bitmap) {
          int bit = bitmap & -bitmap;
          dfs(row + 1, (l | bit) << 1, (r | bit) >> 1, u | bit);
          bitmap ^= bit;
        }
      }
    };
    dfs(0, 0, 0, 0);
    return res;
  }
};
