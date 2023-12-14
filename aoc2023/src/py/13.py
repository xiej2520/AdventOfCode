import heapq
import sys
from collections import defaultdict, Counter
from functools import cache
import math

sys.setrecursionlimit(100000000)

lines = []
for line in sys.stdin:
    lines.append(line.strip())

patterns = [[]]
for line in lines:
    if line == "":
        patterns.append([])
    else:
        patterns[-1].append(line)

res = 0

for p in patterns:
    m, n = len(p), len(p[0])
    for v in range(0, n-1):
        is_vert = True
        for j in range(0, n):
            if v - j < 0 or v + 1 + j >= n:
                #print(v, v - j, v + 1 + j, n)
                break
            for i in range(0, m):
                if p[i][v-j] != p[i][v+1+j]:
                    is_vert = False
                    break
        if is_vert:
            print("v", v)
            res += v + 1
    for h in range(0, m-1):
        is_horiz = True
        for i in range(0, m):
            if h - i < 0 or h + 1 + i >= m:
                break
            for j in range(0, n):
                if p[h-i][j] != p[h+1+i][j]:
                    is_horiz = False
                    break
        if is_horiz:
            print("h", h)
            res += (h + 1) * 100
    #print(p)
    print("--")

print(res)
