#pragma once

// template metaprogramming!
// Credit: Nelson Elhag
// https://blog.nelhage.com/post/advent-of-templates/

template <typename... Elts>
struct list {};

template <auto v>
struct literal {
  constexpr static auto value = v;
};

template <auto... Elt>
struct read_input {
  using type = list<literal<static_cast<char>(Elt)>...>;
};
// xxd -i < 1.in > 1.i

// folds

// Thanks to Gašper Ažman and Joshua Bronson for this faster fold
// implementation.
namespace _f {
template <template <typename, typename> typename Fn, typename T>
struct F;

template <template <typename, typename> typename Fn, typename T>
extern F<Fn, T> Fv;

template <template <typename, typename> typename Fn, typename T>
auto R(F<Fn, T> const &) -> T;

template <template <typename, typename> typename Fn, typename T, typename R>
auto operator<<=(F<Fn, T> const &, F<Fn, R> const &)
    -> F<Fn, typename Fn<T, R>::type> const &;

template <
    template <typename, typename> typename Fn, typename Init, typename List>
struct fold {};
template <
    template <typename, typename> typename Fn, typename Init, typename... Elts>
struct fold<Fn, Init, list<Elts...>> {
  using type = decltype(R((Fv<Fn, Init> <<= ... <<= Fv<Fn, Elts>)));
};
} // namespace _f

using _f::fold;

struct nil;

template <typename T>
struct is_nil {
  using type = literal<false>;
};

template <>
struct is_nil<nil> {
  using type = literal<true>;
};

template <typename Cond, typename Then, typename Else>
struct if_else {};

template <typename Then, typename Else>
struct if_else<literal<true>, Then, Else> {
  using type = Then;
};

template <typename Then, typename Else>
struct if_else<literal<false>, Then, Else> {
  using type = Else;
};

template <typename L, typename R>
struct or_else {
  using type = L;
};

template <typename R>
struct or_else<nil, R> {
  using type = R;
};