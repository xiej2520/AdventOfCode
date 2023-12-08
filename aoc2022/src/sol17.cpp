#include <lib.h>

void print_chamber(span<uint16_t> chamber) {
  for (int i=chamber.size()-1; i>0; i--) {
    char line[] = "|.......|\n";
    for (int j=1; j<8; j++) {
      if (chamber[i] & (1 << j)) line[8 - j] = '#';
    }
    cout << line;
    //cout << bitset<9>(chamber[i]) << '\n';
  }
  cout << "+-------+\n";
}

array<array<uint16_t, 4>, 5> rocks{
    {{{0b000111100, 0b000000000, 0b000000000, 0b000000000}},
     {{0b000010000, 0b000111000, 0b000010000, 0b000000000}},
     {{0b000111000, 0b000001000, 0b000001000, 0b000000000}},
     {{0b000100000, 0b000100000, 0b000100000, 0b000100000}},
     {{0b000110000, 0b000110000, 0b000000000, 0b000000000}}}};

vector<uint16_t> simulate(int n, string_view jet) {
  vector<uint16_t> chamber;
  chamber.push_back(0b111111111);
  chamber.push_back(0b100000001);
  chamber.push_back(0b100000001);
  chamber.push_back(0b100000001);
  chamber.push_back(0b100000001);
  chamber.push_back(0b100000001);
  chamber.push_back(0b100000001);
  chamber.push_back(0b100000001);
  int top = 0;
  int k = 0;
  for (int i=0; i<n; i++) {
    int j = top + 4;
    auto r = rocks[i % 5];
    while (true) {
      array<uint16_t, 4> r_moved;
      // move by jet
      for (int l=0; l<4; l++) {
        if (jet[k] == '<') {
          r_moved[l] = r[l] << 1;
        }
        else {
          r_moved[l] = r[l] >> 1;
        }
      }
      k = (k + 1) % jet.size();
      bool collide = false;
      for (int l=0; l<4; l++) {
        if (r_moved[l] & chamber[j+l]) collide = true;
      }
      if (!collide) r = r_moved;
      // move down
      collide = false;
      j--;
      for (int l=0; l<4; l++) {
        if (r[l] & chamber[j+l]) collide = true;
      }
      if (collide) {
        j++; // undo
        for (int l=0; l<4; l++) {
          chamber[j+l] |= r[l];
        }
        int t;
        switch (i % 5) {
          case 0: t = 0; break;
          case 1: t = 2; break;
          case 2: t = 2; break;
          case 3: t = 3; break;
          case 4: t = 1; break;
        }
        top = max(top, j + t);
        while (chamber.size() < static_cast<size_t>(top) + 8) {
          chamber.push_back(0b100000001);
        }
        break;
      }
    }
  }
  return chamber;
}

// cycle detecting using hashmap
// input is lucky enough where last 8 is good enough? no instances of | on top of
// each other I guess
struct drop_state {
  uint64_t last_8;
  int rock_index;
  int jet_index;
  bool operator==(const drop_state &other) const = default;
};
template<>
struct std::hash<drop_state> {
  size_t operator()(const drop_state &s) const {
    return hash<uint64_t>()(s.last_8) + hash<int>()(s.rock_index) + hash<int>()(s.jet_index);
  }
};

uint64_t height_of(uint64_t n, string_view jet) {
  vector<uint16_t> chamber;
  chamber.push_back(0b111111111);
  chamber.push_back(0b100000001);
  chamber.push_back(0b100000001);
  chamber.push_back(0b100000001);
  chamber.push_back(0b100000001);
  chamber.push_back(0b100000001);
  chamber.push_back(0b100000001);
  chamber.push_back(0b100000001);
  int top = 0;
  int k = 0;

  // last 8 => height, number of rocks
  unordered_map<drop_state, pair<uint64_t, uint64_t>> m;
  uint64_t cycles_height;
  
  
  for (uint64_t i=0; i<n; i++) {
    int j = top + 4;
    auto r = rocks[i % 5];
    while (true) {
      array<uint16_t, 4> r_moved;
      // move by jet
      for (int l=0; l<4; l++) {
        if (jet[k] == '<') {
          r_moved[l] = r[l] << 1;
        }
        else {
          r_moved[l] = r[l] >> 1;
        }
      }
      k = (k + 1) % jet.size();
      bool collide = false;
      for (int l=0; l<4; l++) {
        if (r_moved[l] & chamber[j+l]) collide = true;
      }
      if (!collide) r = r_moved;
      // move down
      collide = false;
      j--;
      for (int l=0; l<4; l++) {
        if (r[l] & chamber[j+l]) collide = true;
      }
      if (collide) {
        j++; // undo
        for (int l=0; l<4; l++) {
          chamber[j+l] |= r[l];
        }
        int t;
        switch (i % 5) {
          case 0: t = 0; break;
          case 1: t = 2; break;
          case 2: t = 2; break;
          case 3: t = 3; break;
          case 4: t = 1; break;
        }
        top = max(top, j + t);
        while (chamber.size() < static_cast<size_t>(top) + 8) {
          chamber.push_back(0b100000001);
        }
        break;
      }
    }
    
    if (i > 10000) {
      uint64_t last_8 = 0;
      for (int l = top; l > top - 8; l--) {
        last_8 = (last_8 << 8) | chamber[l];
      }
      last_8 += i % 5 + k % jet.size();
      drop_state state{last_8, (int) i % 5, (int) k % (int) jet.size()};
      if (auto it = m.find(state); it != m.end()) {
        auto [last_h, last_i] = it->second;
        uint64_t cycle_len = (i + 1) - last_i;
        uint64_t rem = n - i - 1;
        uint64_t div = rem / cycle_len;
        n -= div * cycle_len;
        cycles_height = div * (top - last_h);
        m.clear();
      }
      else {
        m[state] = {top, i + 1};
      }
    }
  }
  return cycles_height + top;
}

void solve_17() {
  string jet;
  cin >> jet;
  
  auto chamber = simulate(2022, jet);
  print_chamber(chamber);
  int t = chamber.size() - 1;
  while (t >= 0 && chamber[t] == 0b100000001) {
    t--;
  }
  
  cout << t << endl;
  cout << height_of(1'000'000'000'000, jet);

}
