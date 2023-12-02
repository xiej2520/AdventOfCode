#include <bits/stdc++.h>
using namespace std;

int main() {
  int n = 3000000;
  cout << n << endl;
  for (int i=0; i<n; i++) {
    string s;
    cout << 'a';
    for (int j=1; j < 43 * i % 97 + 13; j++) {
      s += ('a' + (((((i * 103) % 17) + (j * 53 % 47))* 31) % 26));
    }
    cout << s << "\n";
  }
  for (int i=0; i<n; i++) {
    cout <<  i << "\n";
  }
  cout << fixed << setprecision(8);
  for (int i=0; i<n; i++) {
    double d = i;
    d /= (3 + i);
    d *= (d + i * 3 % 13);
    cout << d << "\n";
  }
  cerr << "done" << endl;

}
