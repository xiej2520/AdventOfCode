import heapq
import sys
from collections import defaultdict, Counter
from functools import cache
import math
from queue import Queue

sys.setrecursionlimit(100000000)

lines = []

for line in sys.stdin:
    lines.append(line.strip())

m, n = len(lines), len(lines[0])

for i in range(m):
    for j in range(n):
        if lines[i][j] == 'S':
            sx = i
            sy = j

q = [(sx, sy)]
for _ in range(64):
    print(q)
    nq = []
    seen = [[False for _ in range(n)] for _ in range(m)]
    while q:
        i, j = q.pop()
        for d in [(-1, 0), (1, 0), (0, -1), (0, 1)]:
            x = i + d[0]
            y = j + d[1]
            if x < 0 or y < 0 or x >= m or y >= m:
                continue
            if lines[x][y] == '#':
                continue
            if seen[x][y]:
                continue
            seen[x][y] = True
            nq.append((x, y))
    q = nq

print(len(q))

res = 0

print(res)