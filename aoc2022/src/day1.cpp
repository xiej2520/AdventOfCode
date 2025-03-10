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
	vector<int> nums;
	priority_queue<int> pq;
	int n;
	int cur = 0;
	string line;
	while (getline(cin, line)) {
		istringstream iss(line);
		if (iss.rdbuf()->in_avail() == 0) {
			nums.push_back(cur);
			pq.push(cur);
			cur = 0;
			n = 0;
		}
		iss >> n;
		cur += n;
	}
	
	int res = 0;
	for (int i : nums) {
		res = max(res, i);
	}
	cout << res << endl;

	res = 0;
	for (int i=0; i<3; i++) {
		res += pq.top();
		pq.pop();
	}
	cout << res << endl;
}
