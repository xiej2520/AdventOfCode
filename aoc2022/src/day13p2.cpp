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
	bool operator==(nlist n) {
		if (holds_alternative<int>(data) && holds_alternative<int>(n.data)) {
			return get<int>(data) == get<int>(n.data);
		}
		else if (holds_alternative<vector<nlist>>(data) && holds_alternative<vector<nlist>>(n.data)) {
			vector<nlist> *v1 = &get<vector<nlist>>(data);
			vector<nlist> *v2 = &get<vector<nlist>>(n.data);
			if (v1->size() != v2->size()) {
				return false;
			}
			for (int i=0; i<v1->size(); i++) {
				if (!((*v1)[i] == (*v2)[i])) {
					return false;
				}
			}
			return true;
		}
		return false;
	}
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
	int res = 1;
	vector<nlist> v;
	while (getline(cin, line1) && getline(cin, line2)) {
		nlist n1 = toList(line1);
		nlist n2 = toList(line2);
		v.push_back(n1);
		v.push_back(n2);
		getline(cin, line1); // \n
	}
	nlist div1 = nlist({nlist{2}});
	nlist div2 = nlist({nlist{6}});
	v.push_back(div1);
	v.push_back(div2);
	sort(v.begin(), v.end(), [](const nlist& n1, const nlist& n2) {
		return dfs(n1, n2) < 0;
	});
	for (int i=0; i<v.size(); i++) {
		if (v[i] == div1) {
			cout << i+1 << " 2" << endl;
			res *= (i+1);
		}
		else if (v[i] == div2) {
			cout << i+1 << " 6" << endl;
			res *= (i+1);
		}
	}
	cout << res << endl;
}
