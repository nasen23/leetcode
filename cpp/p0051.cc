#include "x.h"

class Solution {
  int mask;

public:
  vector<vector<string>> solveNQueens(int n) {
    mask = (1 << n) - 1;
    vector<vector<string>> res;
    vector<int> put(n, 0);
    function<void(int, int, int, int)> dfs = [&](int row, int l, int r, int u) {
      if (row == n) {
        vector<string> ans(n, string(n, '.'));
        for (int i = 0; i < n; i++) {
          ans[i][put[i]] = 'Q';
        }
        res.push_back(ans);
        return;
      }
      int bitmap = mask & ~(l | r | u);
      while (bitmap) {
        int bit = bitmap & -bitmap;
        int t = bit;
        int c = 0;
        while (t) {
          t >>= 1;
          c++;
        }
        put[row] = c - 1;
        dfs(row + 1, (l | bit) << 1, (r | bit) >> 1, u | bit);
        bitmap ^= bit;
      }
    };
    dfs(0, 0, 0, 0);
    return res;
  }
};
