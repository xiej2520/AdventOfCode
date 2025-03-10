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

bool fullcontains(int a1, int a2, int b1, int b2) {
	if (a1 <= b1 && a2 >= b2) {
		return true;
	}
	else if (b1 <= a1 && b2 >= a2) {
		return true;
	}
	return false;
}
bool overlap(int a1, int a2, int b1, int b2) {
	if (a1 <= b1 && a2 >= b1) {
		return true;
	}
	else if (b1 <= a1 && b2 >= a1) {
		return true;
	}
	return false;
}
int main() {
	string line;
	int res = 0;
	int res1 = 0;
	while (getline(cin, line)) {
		vector<string> assigns = split(line, ",");
		vector<string> a1 = split(assigns[0], "-");
		vector<string> a2 = split(assigns[1], "-");
		if (fullcontains(stoi(a1[0]), stoi(a1[1]), stoi(a2[0]), stoi(a2[1]))) {
			res++;
		}
		if (overlap(stoi(a1[0]), stoi(a1[1]), stoi(a2[0]), stoi(a2[1]))) {
			res1++;
		}
	}
	cout << res << " " << res1;
}
