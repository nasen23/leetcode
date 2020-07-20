#include "x.h"

class Solution {
public:
  vector<vector<string>> solveNQueens(int n) {
    vector<vector<string>> res;
    vector<int> put(n, 0);
    function<void(int)> dfs = [&](int r) {
      if (!r) {
        vector<string> ans;
        for (int i = 1; i <= n; i++) {
          string row;
          for (int j = 0; j < n; j++) {
            if (put[j] == i) {
              row.push_back('Q');
            } else {
              row.push_back('.');
            }
          }
          ans.push_back(row);
        }
        res.push_back(ans);
        return;
      }
      for (int i = 0; i < n; i++) {
        if (put[i]) {
          continue;
        }
        bool cont = false;
        for (int j = 1; j <= n - r; j++) {
          if ((i >= j && put[i - j] == r + j) ||
              (i < n - j && put[i + j] == r + j)) {
            cont = true;
            break;
          }
        }
        if (cont) {
          continue;
        }
        put[i] = r;
        dfs(r - 1);
        put[i] = 0;
      }
    };
    dfs(n);
    return res;
  }
};

int main(int argc, char *argv[]) { return 0; }
