import heapq
import sys
from collections import defaultdict, Counter
from functools import cache
import math

sys.setrecursionlimit(100000000)

lines = []
for line in sys.stdin:
    lines.append(list(line.strip()))

m, n = len(lines), len(lines[0])

res = 0

def spin():
    for i in range(m):
        for j in range(n):
            k = i
            if lines[k][j] == 'O':
                while k > 0 and lines[k-1][j] == '.':
                    lines[k][j] = '.'
                    lines[k-1][j] = 'O'
                    k -= 1
    for j in range(n):
        for i in range(m):
            k = j
            if lines[i][k] == 'O':
                while k > 0 and lines[i][k-1] == '.':
                    lines[i][k] = '.'
                    lines[i][k-1] = 'O'
                    k -= 1
    for i in range(m-1,-1,-1):
        for j in range(n):
            k = i
            if lines[k][j] == 'O':
                while k < m - 1 and lines[k+1][j] == '.':
                    lines[k][j] = '.'
                    lines[k+1][j] = 'O'
                    k += 1
    for j in range(n-1,-1,-1):
        for i in range(m):
            k = j
            if lines[i][k] == 'O':
                while k < n - 1 and lines[i][k+1] == '.':
                    lines[i][k] = '.'
                    lines[i][k+1] = 'O'
                    k += 1

seen = {}
i = 0
N = 1000000000
while i < N:
    spin()
    rep = "".join("".join(l) for l in lines)
    if rep in seen:
        cycle_len = i - seen[rep]
        rem = N - i
        div = rem // cycle_len
        N -= div * cycle_len
        print(i, seen[rep], rem, div, N)
    seen[rep] = i
    i += 1

for l in lines:
    print("".join(l))

for i in range(m):
    for j in range(n):
        if lines[i][j] == 'O':
            res += m - i

print(res)
