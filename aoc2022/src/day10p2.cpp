
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

void draw(vector<vector<char>> &CRT, int X, int cycle) {
	int row = (cycle-1)/40;
	int curCol = (cycle-1)%40;
	if (abs(X-curCol) < 2) {
		CRT[row][curCol] = '#';
	}
}

int main() {
	string line;
	int X = 1;
	int cycle = 1;
	vector<vector<char>> CRT(6, vector<char>(40, '.'));
	while (getline(cin, line)) {
		vector<string> ins = split(line, " ");
		if (ins[0] == "noop") {
			draw(CRT, X, cycle);
			cycle++;
		}
		else {
			draw(CRT, X, cycle);
			cycle++;
			draw(CRT, X, cycle);
			cycle++;
			X += stoi(ins[1]);
		}
	}

	for (int i=0; i<CRT.size(); i++) {
		for (int j=0; j<CRT[0].size(); j++) {
			cout << CRT[i][j];
		}
		cout << endl;
	}
}
