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

int main() {
	string line;
	vector<pii> sensors;
	vector<pii> beacons;

	while (getline(cin, line)) {
		vector<string> split_line = split(line, ":");
		vector<string> sensorStr = split(split_line[0], ",");
		sensors.push_back({stoi(sensorStr[0]), stoi(sensorStr[1])});
		vector<string> beaconStr = split(split_line[1], ",");
		beacons.push_back({stoi(beaconStr[0]), stoi(beaconStr[1])});
	}
	const int row = 2000000;
	set<int> beaconsOnRow;
	vector<pii> excludeRanges;
	for (pii& pt : beacons) {
		if (pt.second == row) {
			beaconsOnRow.insert(pt.first);
		}
	}
	for (int i=0; i<sensors.size(); i++) {
		int maxDist = abs(sensors[i].first - beacons[i].first) + abs(sensors[i].second - beacons[i].second);
		if (abs(row - sensors[i].second) <= maxDist) {
			excludeRanges.push_back({
				sensors[i].first - maxDist + abs(row - sensors[i].second),
				sensors[i].first + maxDist - abs(row - sensors[i].second)
			});
		}
	}
	sort(excludeRanges.begin(), excludeRanges.end());

	vector<pii> exRanMerge;
	exRanMerge.push_back(excludeRanges[0]);
	int j = 0;
	for (int i=1; i<excludeRanges.size(); i++) {
		if (excludeRanges[i].first <= exRanMerge[j].second) {
			exRanMerge[j].second = max(exRanMerge[j].second, excludeRanges[i].second);
		}
		else {
			exRanMerge.push_back(excludeRanges[i]);
			j++;
		}
	}
	int res = exRanMerge[0].second - exRanMerge[0].first + 1;
	for (int x : beaconsOnRow) {
		if (x >= exRanMerge[0].first && x <= exRanMerge[0].second) {
			res--;
		}
	}
	cout << res << endl;
}
