#include "lib.h"

vector<string_view> split(string_view s) {
  vector<string_view> res;
  size_t start = 0, end, n = s.size();
  while (start < n && isspace(s[start])) start++;
  while (start < n) {
    end = start + 1;
    while (end < n && !isspace(s[end])) end++;
    res.push_back(s.substr(start, end - start));
    start = end + 1;
    while (start < n && isspace(s[start])) start++;
  }
  return res;
}

vector<string_view> split(string_view s, string_view delim) {
  vector<string_view> res;
  size_t start = 0, end, e = delim.size();
  while ((end = s.find(delim, start)) != string_view::npos) {
    res.push_back(s.substr(start, end - start));
    start = end + e;
  }
  res.push_back(s.substr(start));
  return res;
}
