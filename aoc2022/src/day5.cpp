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

using namespace std;

typedef long long ll;
typedef long double ld;
typedef pair<int, int> pii;
typedef vector<int> vi;
typedef vector<vector<int>> vvi;
typedef unordered_map<int, int> umii;
typedef unordered_map<int, pii>  umipii;
typedef unordered_set<int> usi;

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


int main() {
	string line;
	int res = 0;
	int res1 = 0;
	vector<vector<string>> crates(10);
	for (int i=0; i<9; i++) {
		getline(cin, line);
		crates[i+1] = split(line, " ");
	}
	vector<vector<string>> crates2 = crates;
	for (int i=0; i<10; i++) {
		next(crates2[i].end(), 100);
	}
	while (getline(cin, line)) {
		vector<string> vline = split(line, " ");
		int from = stoi(vline[3]);
		int to = stoi(vline[5]);
		for (int i=0; i<stoi(vline[1]); i++) {
			crates[to].push_back(crates[from].back());
			crates[from].pop_back();
		}
		int n = stoi(vline[1]);
		for (int i=0; i<n; i++) {
			crates2[to].push_back(crates2[from][crates2[from].size()-n+i]);
		}
		for (int i=0; i<n; i++) {
			crates2[from].pop_back();
		}
	}
	string sres = "";
	string sres1 = "";
	for (int i=1; i<10; i++) {
		sres += crates[i].back();
		sres1 += crates2[i].back();
	}
	cout << sres << "\n";
	cout << sres1 << "\n";
}
