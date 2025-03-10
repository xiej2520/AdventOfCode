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
	cin >> line;
	int res = 0;

	unordered_map<char, int> m;
	int l = 14;
	
	for (int i=0; i<l; i++) {
		m[line[i]]++;
	}
	for (int i=l; i<line.size(); i++) {
		m[line[i-l]]--;
		if (m[line[i-l]] == 0) {
			m.erase(line[i-l]);
		}
		m[line[i]]++;
		if (m.size() == l) {
			res = i+1;
			break;
		}
	}
	cout << res << "\n";
}
