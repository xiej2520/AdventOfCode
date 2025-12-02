#!/usr/bin/env bash

set -e

if [ -z "$1" ]; then
  cat <<EOF
    Usage: $0 <filename_no_prefix> [options]
    Options:
      -no-lib    Don't add lib.ll
EOF
  exit 1
fi

for arg in "$@"; do
  if [[ "$arg" == "-no-lib" ]]; then
    no_lib=true
  fi
done

mkdir -p target

if [[ "$no_lib" == true ]]; then
  llc src/${1}.ll -o target/$1.s
else
  cat src/$1.ll src/lib.ll > target/${1}_with_lib.ll
  llc target/${1}_with_lib.ll -o target/$1.s
fi

as target/$1.s -o target/$1.o
ld target/$1.o -o target/$1
