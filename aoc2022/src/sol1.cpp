#include "tmp_lib.h"

using problem = read_input<
#include "../inputs/1.i"
    >::type;

// part 1

template <typename MaxElves, typename CurElf, typename CurNum>
struct State1 {
  using max_elves = MaxElves;
  using cur_elf = CurElf;
  using cur_num = CurNum;
};

using empty_state1 = State1<literal<0>, literal<0>, literal<0>>;

template <typename T>
using as_digit = if_else<
    literal<T::value >= '0' && T::value <= '9'>, literal<T::value - '0'>, nil>;

template <typename T, typename S>
using max = typename if_else<literal<(T::value > S::value)>, T, S>::type;

// (In, Char) -> { type: State }
template <typename In, typename Char>
struct ReducePart1 {
  using digit = typename as_digit<Char>::type;

  struct NewlineSepElf {
    using type = State1<
        max<typename In::max_elves, typename In::cur_elf>, literal<0>,
        literal<0>
    >;
  };
  struct NewlineSepItem {
    using type = State1<
        typename In::max_elves,
        literal<In::cur_elf::value + In::cur_num::value>, literal<0>
    >;
  };

  struct AddDigit {
    using type = State1<
        typename In::max_elves, typename In::cur_elf,
        literal<In::cur_num::value * 10 + digit::value>>;
  };

  using type = typename if_else<
      literal<Char::value == '\n'>,
      typename if_else<literal<In::cur_num::value == 0>,
          NewlineSepElf,
          NewlineSepItem
      >::type,
      AddDigit
      >::type::type;
};

template <typename T>
struct solve1 {
  using type = typename fold<ReducePart1, empty_state1, T>::type::max_elves;
};

// part 2

template <typename T>
using as_digit = if_else<
    literal<T::value >= '0' && T::value <= '9'>, literal<T::value - '0'>, nil>;

template <typename Elf1, typename Elf2, typename Elf3, typename CurElf, typename CurNum>
struct State2 {
  using elf_1 = Elf1;
  using elf_2 = Elf2;
  using elf_3 = Elf3;
  using cur_elf = CurElf;
  using cur_num = CurNum;
};

using empty_state2 = State2<literal<0>, literal<0>, literal<0>, literal<0>, literal<0>>;

template <typename In, typename Char>
struct ReducePart2 {
  using digit = typename as_digit<Char>::type;

  struct NewlineSepElf {
    using type = typename if_else<literal<(In::cur_elf::value > In::elf_1::value)>,
      State2<
        typename In::cur_elf,
        typename In::elf_1,
        typename In::elf_2,
        literal<0>,
        literal<0>
      >,
      typename if_else<literal<(In::cur_elf::value > In::elf_2::value)>,
        State2<
          typename In::elf_1,
          typename In::cur_elf,
          typename In::elf_2,
          literal<0>,
          literal<0>
        >,
        typename if_else<literal<(In::cur_elf::value > In::elf_3::value)>,
          State2<
            typename In::elf_1,
            typename In::elf_2,
            typename In::cur_elf,
            literal<0>,
            literal<0>
          >,
          State2<
            typename In::elf_1,
            typename In::elf_2,
            typename In::elf_3,
            literal<0>,
            literal<0>
          >
        >::type
      >::type
    >::type;
  };


  struct NewlineSepItem {
    using type = State2<
        typename In::elf_1,
        typename In::elf_2,
        typename In::elf_3,
        literal<In::cur_elf::value + In::cur_num::value>,
        literal<0>
    >;
  };

  struct AddDigit {
    using type = State2<
        typename In::elf_1,
        typename In::elf_2,
        typename In::elf_3,
        typename In::cur_elf,
        literal<In::cur_num::value * 10 + digit::value>
    >;
  };

  using type = typename if_else<
      literal<Char::value == '\n'>,
      typename if_else<literal<In::cur_num::value == 0>,
          NewlineSepElf,
          NewlineSepItem
      >::type,
      AddDigit
  >::type::type;
};

template<typename T>
struct solve2 {
  using res = typename fold<ReducePart2, empty_state2, T>::type;
  using type = literal<res::elf_1::value + res::elf_2::value + res::elf_3::value>;
};

#include <cstdio>

void solve_1() {
  using res_1 = solve1<problem>::type;
  using res_2 = solve2<problem>::type;
  printf("%d\n%d\n", res_1::value, res_2::value);
}
