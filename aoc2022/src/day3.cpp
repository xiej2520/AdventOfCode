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

int main() {

	/*
	string line;
	int res = 0;
	while (getline(cin, line)) {
		int n = line.size();
		usi s;
		for (int i=0; i<n/2; i++) {
			s.insert(line[i]);
		}
		for (int i=n/2; i<n; i++) {
			if (s.count(line[i])) {
				cout << line[i] << " " << (line[i] - 'a') << " " << (line[i] - 'A') << endl;
				if (line[i] >= 'a') {
					res += line[i] - 'a' + 1;
				}
				else {
					res += line[i] - 'A' + 27;
				}
				break;
			}
		}
	}
	*/
	string line;
	int res = 0;
	for (int i=0; i<100; i++) {
		vector<int> c(255, 0);
		getline(cin, line);
		for (int j=0; j<line.size(); j++) {
			if (c[line[j]] == 0) {
				c[line[j]]++;
			}
		}
		getline(cin, line);
		for (int j=0; j<line.size(); j++) {
			if (c[line[j]] == 1)
				c[line[j]]++;
		}
		getline(cin, line);
		for (int j=0; j<line.size(); j++) {
			if (c[line[j]] == 2)
				c[line[j]]++;
		}
		for (int j=0; j<255; j++) {
			if (c[j] == 3) {
				if (j >= 'a') {
					res += j - 'a' + 1;
				}
				else {
					res += j - 'A' + 27;
				}
			}
		}
	}
	cout << res << endl;
}
