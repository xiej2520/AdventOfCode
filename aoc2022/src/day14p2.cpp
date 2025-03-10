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
int drop(vector<vector<bool>>& tiles, pii coord, int maxY) {
	if (coord.second == maxY-1) {
		tiles[coord.first][coord.second] = true;
		return coord.second;
	}
	if (!tiles[coord.first][coord.second+1]) {
		return drop(tiles, {coord.first, coord.second+1}, maxY);
	}
	else if (!tiles[coord.first-1][coord.second+1]) {
		return drop(tiles, {coord.first-1, coord.second+1}, maxY);
	}
	else if (!tiles[coord.first+1][coord.second+1]) {
		return drop(tiles, {coord.first+1, coord.second+1}, maxY);
	}
	else {
		tiles[coord.first][coord.second] = true;
		return coord.second;
	}
	return INT_MIN;
}

int main() {
	string line;
	vector<pii> pts;

	while (getline(cin, line)) {
		vector<string> corners = split(line, " -> ");
		vector<string> sprev = split(corners[0], ",");
		pii prev = {stoi(sprev[0]), stoi(sprev[1])};
		for (string& pt : corners) {
			vector<string> scur = split(pt, ",");
			pii cur = {stoi(scur[0]), stoi(scur[1])};
			while (prev != cur) {
				pts.push_back(prev);
				if (prev.first != cur.first) {
					prev.first += prev.first > cur.first ? -1 : 1;
				}
				else {
					prev.second += prev.second > cur.second ? -1 : 1;
				}
			}
			pts.push_back(cur);
			prev = cur;
		}
	}
	int minX, minY = INT_MAX;
	int maxX, maxY = INT_MIN;
	for (pii pt : pts) {
		minX = min(minX, pt.first);
		maxX = max(maxX, pt.first);
		minY = min(minY, pt.second);
		maxY = max(maxY, pt.second);
	}
	cout << minX << " " << maxX << " " << minY << " " << maxY << endl;
	
	vector<vector<bool>> tiles = vector<vector<bool>>(1000, vector<bool>(400, 0));
	for (pii pt : pts) {
		tiles[pt.first][pt.second] = true;
	}
	maxY += 2;
	int res = 0;
	int prev = INT_MIN;
	while (prev != 0) {
		prev = drop(tiles, {500, 0}, maxY);
		res++;
	}

	cout << res << " " << prev << endl;
}
