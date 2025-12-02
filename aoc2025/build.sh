#!/usr/bin/env bash

set -e

if [ -z "$1" ]; then
  echo "usage: ./build.sh <file name no prefix>"
  exit 1
fi


llc src/$1.ll -o target/$1.s
as target/$1.s -o target/$1.o
ld target/$1.o -o target/$1
