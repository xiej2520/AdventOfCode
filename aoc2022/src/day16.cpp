#include <iostream>
#include <algorithm>
#include <string>
#include <vector>
#include <queue>
#include <deque>
#include <map>
#include <set>
#include <unordered_map>
#include <unordered_set>
#include <stack>
#include <numeric>
#include <sstream>
#include <functional>
#include <variant>
using namespace std;

typedef long long ll;
typedef long double ld;
typedef pair<int, int> pii;
typedef vector<int> vi;
typedef vector<vector<int>> vvi;
typedef unordered_map<int, int> umii;
typedef unordered_map<int, pii>  umipii;
typedef unordered_set<int> usi;
template<class T>
using us = unordered_set<T>;
template<class T, class U>
using um = unordered_map<T, U>;

vector<string> split(const string &s, string delim) {
	vector<string> res;
	size_t start = 0, end, len = delim.size();
	string token;
	while ((end = s.find(delim, start)) != string::npos) {
		token = s.substr(start, end-start);
		start = end + len;
		res.push_back(token);
	}
	res.push_back(s.substr(start));
	return res;
}

int maxRelease = 0;

int numNodes = 0;
int numNonZero = 0;
um<int, int> rates;
um<string, int> id;
vector<int> useful;
um<int, int> nToUseful;
vector<um<string, int>> dp(31);
// dp[i][m] = pressure that can be released from time i with m opened at time i
string fullMask;

int dfs(vvi& shortestPath, int node, string mask, int t) {
	if (t >= 30) {
		return 0;
	}
	if (dp[t].count(mask)) {
		return dp[t][mask];
	}
	int m = 0;
	for (int i : useful) {
		if (mask[nToUseful[i]] == '0') {
			int t_next = t + shortestPath[node][i] + 1;
			if (t_next < 30) {
				int released = (30 - t_next) * rates[i];
				string mask_next = mask;
				mask_next[nToUseful[i]] = '1';
				m = max(m, released + dfs(shortestPath, i, mask_next, t_next));
			}
		}
	}
	return dp[t][mask] = m;
}

int main() {
	string line;
	um<string, us<string>> graph;

	while (getline(cin, line)) {
		string start = line.substr(6, 2);
		id[start] = numNodes;
		vector<string> splits = split(line, ";");
		int rate = stoi(splits[0].substr(23));
		if (rate != 0) {
			useful.push_back(numNodes);
			nToUseful[numNodes] = numNonZero;
			numNonZero++;
		}
		rates[numNodes] = rate;
		string linkstr = splits[1][7] == 's' ? splits[1].substr(24) : splits[1].substr(23);
		vector<string> linksSplit = split(linkstr, ", ");
		for (string node : linksSplit) {
			graph[start].insert(node);
		}
		numNodes++;
	}
	fullMask = string(numNonZero, '1');

	vector<vector<int>> shortestPath(numNodes, vector<int>(numNodes, 100));
	for (auto& p : graph) {
		for (string next : p.second) {
			shortestPath[id[p.first]][id[next]] = 1;
		}
	}
	// Floyd-Warshall
	for (int k=0; k<numNodes; k++) {
		for (int i=0; i<numNodes; i++) {
			for (int j=0; j<numNodes; j++) {
				shortestPath[i][j] = min(shortestPath[i][j],
					shortestPath[i][k] + shortestPath[k][j]);
			}
		}
	}

	string mask(numNonZero, '0'); // bitstring
	cout << dfs(shortestPath, id["AA"], mask, 0) << endl;

}
