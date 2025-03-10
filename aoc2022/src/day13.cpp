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

vector<string> split(const string &s, string delims) {
	vector<string> res;
	size_t start = 0, end;
	string token;
	while ((end = s.find_first_of(delims, start)) != string::npos) {
		if (start == end) {
			res.push_back(s.substr(start, 1));
			start++;
		}
		else {
			token = s.substr(start, end - start);
			start = end;
			res.push_back(token);
		}
	}
	if (s.substr(start) != "") {
		res.push_back(s.substr(start));
	}
	return res;
}

struct nlist {
	variant<int, vector<nlist>> data;
	nlist(int d): data(d) {}
	nlist() { data = vector<nlist>{}; }
};

nlist toList(string s) {
	stack<nlist*> st;
	vector<string> tokens = split(s, "[,]");
	st.push(new nlist());
	for (string tk : tokens) {
		if (tk == "[") {
			//cout << "+1" << endl;
			get<vector<nlist>>(st.top()->data).push_back(nlist{});
			st.push(&(get<vector<nlist>>(st.top()->data).back()));
		}
		else if (tk == "]") {
			//cout << "-1" << endl;
			st.pop();
		}
		else if (tk != ",") {
			get<vector<nlist>>(st.top()->data).push_back(nlist(stoi(tk)));
		}
	}
	return *st.top();
}

int dfs(nlist n1, nlist n2) {
	if (holds_alternative<int>(n1.data) && holds_alternative<int>(n2.data)) {
		if (get<int>(n1.data) == get<int>(n2.data)) {
			return 0;
		}
		return get<int>(n1.data) < get<int>(n2.data) ? -1 : 1;
	}
	if (holds_alternative<int>(n1.data)) {
		int v = get<int>(n1.data);
		n1 = nlist{};
		get<vector<nlist>>(n1.data).push_back(v);
	}
	if (holds_alternative<int>(n2.data)) {
		int v = get<int>(n2.data);
		n2 = nlist{};
		get<vector<nlist>>(n2.data).push_back(v);
	}
	vector<nlist> *v1 = &get<vector<nlist>>(n1.data);
	vector<nlist> *v2 = &get<vector<nlist>>(n2.data);
	int k = min(v1->size(), v2->size());
	for (int i=0; i<k; i++) {
		int r = dfs((*v1)[i], (*v2)[i]);
		if (r != 0) {
			return r;
		}
	}
	if (v1->size() == v2->size()) {
		return 0;
	}
	return v1->size() < v2->size() ? -1 : 1;
}

int main() {
	string line1;
	string line2;
	int i = 1;
	int res = 0;
	while (getline(cin, line1) && getline(cin, line2)) {
		cout << i << endl;
		
		nlist n1 = toList(line1);
		nlist n2 = toList(line2);
		if (dfs(n1, n2) <= 0) {
			res += i;
		}
		i++;
		getline(cin, line1); // \n
	}
	cout << res << endl;
}
