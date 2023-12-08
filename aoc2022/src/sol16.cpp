#include "lib.h"

void solve_16() {
  string line;
  vector<string> lines;
  array<int, 676> ids;
  vector<int> flow_rate;
  int n = 0;
/*"
Valve TM has flow rate=0; tunnels lead to valves KF, AA
"*/
  while (getline(cin, line)) {
    ids[26 * (line[6] - 'A') + (line[7] - 'A')] = n;
    n++;
    int j = 24;
    while (line[j] != ';') j++;
    flow_rate.push_back(stoi(line.substr(23, j-23)));
    lines.push_back(std::move(line));
  }
  vector<vector<int>> D(n, vector<int>(n, INT_MAX / 3));
  int AA_index;
  for (int i=0; i<n; i++) {
    if (lines[i][6] == 'A' && lines[i][7] == 'A') AA_index = i;
    auto valves_loc = lines[i].find("valves") + 7;
    auto valves = split(string_view(lines[i]).substr(valves_loc), ", ");
    for (auto v : valves) {
      int j = ids[26 * (v[0] - 'A') + (v[1] - 'A')];
      D[i][j] = 1;
      D[j][i] = 1;
    }
  }
  
  // Floyd-Warshall
  for (int k=0; k<n; k++)
    for (int i=0; i<n; i++)
      for (int j=0; j<n; j++)
        D[i][j] = min(D[i][j], D[i][k] + D[k][j]);
  
  vector<vector<int>> E;
  vector<int> flow_pos;
  for (int i=0; i<n; i++) {
    if (flow_rate[i] == 0) continue;
    flow_pos.push_back(flow_rate[i]);
    vector<int> dists;
    for (int j=0; j<n; j++) {
      if (flow_rate[j] == 0) continue;
      dists.push_back(D[i][j]);
    }
    E.push_back(std::move(dists));
  }

  int m = E.size();

  auto solve_t = [&](int t) {
    // dp[i][mask]: max pressure released at valve i after mask visited
    vector<vector<int>> dp(m, vector<int>(1 << m, 0));
    // t remaining, released is total released (to 30 min) from visited
    auto dfs = [&](auto &&rec, int i, int visited, int t, int released) -> void {
      for (int j=0; j<m; j++) {
        if (visited & (1 << j)) continue;
        if (t - E[i][j] < 0) continue;
        int t_remain = t - E[i][j] - 1;
        int released_j = released + t_remain * flow_pos[j];
        int visited_j = visited | (1 << j);
        if (released_j <= dp[j][visited_j]) continue;
        dp[j][visited_j] = released_j;
        rec(rec, j, visited_j, t_remain, released_j);
      }
    };
    
    // start from node AA, which isn't one of the pos flow
    int i = 0; // index of node with pos flow
    for (int j=0; j<n; j++) {
      if (flow_rate[j] != 0) {
        int t_remain = t - D[AA_index][j] - 1;
        int released_i = t_remain * flow_pos[i];
        dp[i][1 << i] = released_i;
        dfs(dfs, i, 1 << i, t_remain, released_i);
        i++;
      }
    }

    vector<int> max_mask(1<<m);
    for (auto &v : dp) {
      for (int i=0; i<(1<<m); i++) {
        max_mask[i] = max(max_mask[i], v[i]);
      }
    }
    return max_mask;
  };

  int res_1 = 0;
  for (auto i : solve_t(30)) {
    res_1 = max(res_1, i);
  }

  vector<int> max_mask2 = solve_t(26);
  
  int res_2 = 0;
  for (int i=0; i < (1<<m); i++) {
    if (max_mask2[i] == 0) continue;
    // go through all complementary masks
    int mask = (~i) & ((1<<m) - 1);
    int j = 0;
    do {
      res_2 = max(res_2, max_mask2[i] + max_mask2[j]);
      // iterate through all combinations of bits in mask
      j = (j - mask) & mask;
    } while (j != 0);
  }

  printf("%d\n%d\n", res_1, res_2);
}
