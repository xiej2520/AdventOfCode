#include "lib.h"

int main(int argc, char **argv) {
  int sol_num;
  if (argc < 2) {
    printf("Specify a problem number\n");
    return 0;
  }
  sol_num = stoi(argv[1]);
  auto input_file = string("inputs/") + (argc >= 3 ? argv[2] : argv[1]) + ".in";
  
  ifstream in(input_file);
  cin.rdbuf(in.rdbuf());
  
  switch (sol_num) {
    case 16: solve_16(); break;
  }
}
