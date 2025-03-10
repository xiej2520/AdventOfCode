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
	while (getline(cin, line)) {
		vi row;
		for (int i=0; i<line.size(); i++) {
			row.push_back(line[i] - '0');
		}
		trees.push_back(row);
	}
	vector<vector<bool>> visible(trees.size(), vector<bool>(trees[0].size(), false));
	int res = 0;
	for (int i=0; i<trees.size(); i++) {
		int maxH = -1;
		for (int j=0; j<trees[0].size(); j++) {
			if (trees[i][j] > maxH) {
				maxH = trees[i][j];
				if (!visible[i][j]) {
					res++;
					visible[i][j] = true;
				}
			}
		}
		maxH = -1;
		for (int j=trees[0].size()-1; j>=0; j--) {
			if (trees[i][j] > maxH) {
				maxH = trees[i][j];
				if (!visible[i][j]) {
					res++;
					visible[i][j] = true;
				}
			}
		}
	}
	for (int j=0; j<trees[0].size(); j++) {
		int maxH = -1;
		for (int i=0; i<trees.size(); i++) {
			if (trees[i][j] > maxH) {
				maxH = trees[i][j];
				if (!visible[i][j]) {
					res++;
					visible[i][j] = true;
				}
			}
		}
		maxH = -1;
		for (int i=trees.size()-1; i>=0; i--) {
			if (trees[i][j] > maxH) {
				maxH = trees[i][j];
				if (!visible[i][j]) {
					res++;
					visible[i][j] = true;
				}
			}
		}
	}
	cout << res << endl;
	vvi viewL;
	vvi viewR;
	vvi viewU;
	vvi viewD;
	int n = trees.size();
	int m = trees[0].size();
	for (int i=0; i<n; i++) {
		vector<int> maxStackL;
		vector<int> viewLc;
		// [viewing distance, index of tree taller/equal to it]
		for (int j=0; j<m; j++) {
			while (!maxStackL.empty() && trees[i][j] > trees[i][maxStackL.back()]) {
				maxStackL.pop_back();
			}
			if (maxStackL.empty()) {
				viewLc.push_back(j);
			}
			else {
				viewLc.push_back(j - maxStackL.back());
			}
			maxStackL.push_back(j);
		}
		viewL.push_back(viewLc);
		vector<int> maxStackR;
		vector<int> viewRc;
		for (int j=m-1; j>=0; j--) {
			while (!maxStackR.empty() && trees[i][j] > trees[i][maxStackR.back()]) {
				maxStackR.pop_back();
			}
			if (maxStackR.empty()) {
				viewRc.push_back(m-j-1);
			}
			else {
				viewRc.push_back(maxStackR.back() - j);
			}
			maxStackR.push_back(j);
		}
		reverse(viewRc.begin(), viewRc.end());
		viewR.push_back(viewRc);
	}
	for (int j=0; j<m; j++) {
		vector<int> maxStackU;
		vector<int> viewUc;
		for (int i=0; i<n; i++) {
			while (!maxStackU.empty() && trees[i][j] > trees[maxStackU.back()][j]) {
				maxStackU.pop_back();
			}
			if (maxStackU.empty()) {
				viewUc.push_back(i);
			}
			else {
				viewUc.push_back(i - maxStackU.back());
			}
			maxStackU.push_back(i);
		}
		viewU.push_back(viewUc);
		vector<int> maxStackD;
		vector<int> viewDc;
		for (int i=n-1; i>=0; i--) {
			while (!maxStackD.empty() && trees[i][j] > trees[maxStackD.back()][j]) {
				maxStackD.pop_back();
			}
			if (maxStackD.empty()) {
				viewDc.push_back(n-i-1);
			}
			else {
				viewDc.push_back(maxStackD.back() - i);
			}
			maxStackD.push_back(i);
		}
		reverse(viewDc.begin(), viewDc.end());
		viewD.push_back(viewDc);
	}
	int res1 = 0;

	for (int i=0; i<m; i++) {
		for (int j=0; j<n; j++) {
			//cout << viewD[j][i] << " ";
			//cout << trees[i][j] << " " << viewL[i][j] << " " << viewR[i][j] << " " << viewU[j][i] << " " << viewD[j][i] << endl;
			res1 = max(res1, viewL[i][j] * viewR[i][j] * viewU[j][i] * viewD[j][i]);
		}
	}
	cout << res1 << endl;
}
