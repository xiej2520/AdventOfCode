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

struct File {
	string name;
	int size;
};
struct Dir {
	string name;
	int size = -1;
	Dir* parent;
	unordered_map<string, Dir *> links;
	unordered_map<string, File> files;
};

void print(Dir &d) {
	cout << "Directory: " << d.name << "\n";
	for (auto &p: d.files) {
		cout << p.first << " " << p.second.size << " ";
	}
	cout << "\n";
	for (auto &p : d.links) {
		print(*p.second);
	}
}

int res = 0;

int dfs(Dir &d) {
	if (d.size > 0) {
		return d.size;
	}
	int s = 0;
	for (auto &p : d.links) {
		s += dfs(*p.second);
	}
	for (auto &p : d.files) {
		s += p.second.size;
	}
	d.size = s;
	if (s <= 100000) {
		res += s;
	}
	return s;
}

int res2 = 1e9;
int MIN_DEL = 8381165;
void dfs2(Dir &d) {
	if (d.size >= MIN_DEL) {
		res2 = min(res2, d.size);
		for (auto &p : d.links) {
			dfs2(*p.second);
		}	
	}
}

int main() {
	string line;
	Dir root;
	root.name = '/';
	getline(cin, line);
	Dir* current = &root;
	while (getline(cin, line)) {
		vector<string> vl = split(line, " ");
		if (vl[0] == "$") {
			if (vl[1] == "cd") {
				if (vl[2] == "..") {
					current = current->parent;
				}
				else {
					current = current->links[vl[2]];
				}
			}
		}
		else if (vl[0] == "dir") {
			Dir* d = new Dir;
			d->parent = current;
			d->name = vl[1];
			current->links[vl[1]] = d;
		}
		else {
			File f;
			f.name = vl[1];
			f.size = stoi(vl[0]);
			current->files[vl[1]] = f;
		}
	}
	cout << dfs(root) << endl;
	cout << res << endl;
	MIN_DEL = 30000000 - (70000000 - root.size);
	cout << "needed space: " << MIN_DEL << endl;
	dfs2(root);
	cout << "root size" << root.size << endl;
	cout << res2 << endl;
	//print(root);
}
