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
	int res = 0;
	int res2 = 0;
	string line;
	while (getline(cin, line)) {
		istringstream iss(line);
		char c, d;
		iss >> c >> d;
		if (d == 'X') {
			res += 1;
			if (c == 'A') {
				res += 3;
			}
			else if (c == 'C') {
				res += 6;
			}
		}
		else if (d == 'Y') {
			res += 2;
			if (c == 'B') {
				res += 3;
			}
			else if (c == 'A') {
				res += 6;
			}
		}
		else {
			res += 3;
			if (c == 'C') {
				res += 3;
			}
			else if (c == 'B') {
				res += 6;
			}
		}
		
		if (c == 'A') {
			if (d == 'X') {
				res2 += 3;
			}
			else if (d == 'Y') {
				res2 += 3;
				res2 += 1;
			}
			else {
				res2 += 6;
				res2 += 2;
			}
		}
		else if (c == 'B') {
			if (d == 'X') {
				res2 += 1;
			}
			else if (d == 'Y') {
				res2 += 3;
				res2 += 2;
			}
			else {
				res2 += 6;
				res2 += 3;
			}
		}
		else if (c == 'C') {
			if (d == 'X') {
				res2 += 2;
			}
			else if (d == 'Y') {
				res2 += 3;
				res2 += 3;
			}
			else {
				res2 += 6;
				res2 += 1;
			}
		}
	}
	cout << res << endl;
	cout << res2 << endl;
}
