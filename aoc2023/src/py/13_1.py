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
    vdiffs = []
    for v in range(0, n-1):
        diffs = 0
        for j in range(0, n):
            if v - j < 0 or v + 1 + j >= n:
                #print(v, v - j, v + 1 + j, n)
                break
            for i in range(0, m):
                if p[i][v-j] != p[i][v+1+j]:
                    diffs += 1
        vdiffs.append((v, diffs))
    hdiffs = []
    for h in range(0, m-1):
        diffs = 0
        for i in range(0, m):
            if h - i < 0 or h + 1 + i >= m:
                break
            for j in range(0, n):
                if p[h-i][j] != p[h+1+i][j]:
                    diffs += 1
        hdiffs.append((h, diffs))
    vdiffs.sort(key=lambda x:x[1])
    hdiffs.sort(key=lambda x:x[1])
    print(vdiffs)
    print(hdiffs)
    while vdiffs and vdiffs[0][1] == 0:
        del vdiffs[0]
    if vdiffs and vdiffs[0][1] == 1:
        res += vdiffs[0][0] + 1
    else:
        while hdiffs and hdiffs[0][1] == 0:
            del hdiffs[0]
        res += 100 * (hdiffs[0][0] + 1)
    #print(p)
    print("--")

print(res)
