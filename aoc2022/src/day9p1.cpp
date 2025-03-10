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
	vvi trees;
	pii head = {0, 0};
	pii tail = {0, 0};
	unordered_map<int, usi> visited;
	visited[0].insert(0);
	string dir;
	int steps;
	while (getline(cin, line)) {
		istringstream iss(line);
		iss >> dir >> steps;
		for (int i=0; i<steps; i++) {
			if (dir == "U") {
				head.first--;
			}
			else if (dir == "D") {
				head.first++;
			}
			else if (dir == "L") {
				head.second--;
			}
			else if (dir == "R") {
				head.second++;
			}
			
			if (head.first == tail.first-2 && head.second == tail.second-1) {
				tail.first--;
				tail.second--;
			}
			else if (head.first == tail.first-2 && head.second == tail.second+1) {
				tail.first--;
				tail.second++;
			}
			else if (head.first == tail.first+2 && head.second == tail.second-1) {
				tail.first++;
				tail.second--;
			}
			else if (head.first == tail.first+2 && head.second == tail.second+1) {
				tail.first++;
				tail.second++;
			}
			else if (head.first == tail.first-1 && head.second == tail.second-2) {
				tail.first--;
				tail.second--;
			}
			else if (head.first == tail.first-1 && head.second == tail.second+2) {
				tail.first--;
				tail.second++;
			}
			else if (head.first == tail.first+1 && head.second == tail.second-2) {
				tail.first++;
				tail.second--;
			}
			else if (head.first == tail.first+1 && head.second == tail.second+2) {
				tail.first++;
				tail.second++;
			}
			else if (head.first == tail.first-2) {
				tail.first--;
			}
			else if (head.first == tail.first+2) {
				tail.first++;
			}
			else if (head.second == tail.second-2) {
				tail.second--;
			}
			else if (head.second == tail.second+2) {
				tail.second++;
			}
			visited[tail.first].insert(tail.second);
		}
	}
	int res = 0;
	for (auto &p : visited) {
		res += p.second.size();
	}
	cout << res << endl;
}
