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
	vvi map;
	string line;
	pii start;
	pii end;
	int r = 0;
	while (getline(cin, line)) {
		vector<int> row;
		for (int i=0; i<line.size(); i++) {
			if (line[i] == 'E') {
				row.push_back('z' - 'a');
				end.first = r;
				end.second = i;
			}
			else if (line[i] == 'S') {
				row.push_back(0);
				start.first = r;
				start.second = i;
			}
			else {
				row.push_back(line[i] - 'a');
			}
		}
		r++;
		map.push_back(row);
	}
	deque<pii> next;
	next.push_back(end);
	int i = 0;
	int m = map.size();
	int n = map[0].size();
	while (true) {
		int oS = next.size();
		for (int j=0; j<oS; j++) {
			pii cur = next.front();
			if (map[cur.first][cur.second] == 0) {
				cout << i << endl;
				goto end;
			}
			next.pop_front();
			if (map[cur.first][cur.second] < 0) {
				continue;
			}
			if (cur.first > 0 && map[cur.first-1][cur.second] - map[cur.first][cur.second] >= -1) {
				next.push_back({cur.first-1, cur.second});
			}
			if (cur.first < m-1 && map[cur.first+1][cur.second] - map[cur.first][cur.second] >= -1) {
				next.push_back({cur.first+1, cur.second});
			}
			if (cur.second > 0 && map[cur.first][cur.second-1] - map[cur.first][cur.second] >= -1) {
				next.push_back({cur.first, cur.second-1});
			}
			if (cur.second < n-1 && map[cur.first][cur.second+1] - map[cur.first][cur.second] >= -1) {
				next.push_back({cur.first, cur.second+1});
			}
			map[cur.first][cur.second] = -1;
		}
		i++;
	}
	end:
	cout << i << endl;

}
