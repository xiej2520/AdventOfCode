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
#define int long long
	string line;
	vector<deque<int>> mItems = {
		{72, 64, 51, 57, 93, 97, 68},
		{62},
		{57, 94, 69, 79, 72},
		{80, 64, 92, 93, 64, 56},
		{70, 88, 95, 99, 78, 72, 65, 94},
		{57, 95, 81, 61},
		{79, 99},
		{68, 98, 62}
	};
	vector<function<int(int)>> mInspect = {
		[](int o) { return o * 19; },
		[](int o) { return o * 11; },
		[](int o) { return o + 6; },
		[](int o) { return o + 5; },
		[](int o) { return o + 7; },
		[](int o) { return o * o; },
		[](int o) { return o + 2; },
		[](int o) { return o + 3; }
	};
	vector<function<int(int)>> mThrow = {
		[](int o) { return o % 17 == 0 ? 4 : 7; },
		[](int o) { return o % 3 == 0 ? 3 : 2; },
		[](int o) { return o % 19 == 0 ? 0 : 4; },
		[](int o) { return o % 7 == 0 ? 2 : 0; },
		[](int o) { return o % 2 == 0 ? 7 : 5; },
		[](int o) { return o % 5 == 0 ? 1 : 6; },
		[](int o) { return o % 11 == 0 ? 3 : 1; },
		[](int o) { return o % 13 == 0 ? 5 : 6; }
	};
	
	vector<int> mInspectTimes(8, 0);
	int MOD = 2 * 3 * 5 * 7 * 11 * 13 * 17 * 19;
	for (int i=0; i<10000; i++) {
		for (int j=0; j<mItems.size(); j++) {
			while (!mItems[j].empty()) {
				mInspectTimes[j]++;
				int item = mItems[j].front();
				item = mInspect[j](item);
				item %= MOD;
				mItems[mThrow[j](item)].push_back(item);
				mItems[j].pop_front();
			}
		}
	}
	sort(mInspectTimes.begin(), mInspectTimes.end());
	for (int i=0; i<mInspectTimes.size(); i++) {
		cout << mInspectTimes[i] << endl;
	}
	int n = mInspectTimes.size();
	int res = mInspectTimes[n-1] * mInspectTimes[n-2];
	cout << res << endl;
}
