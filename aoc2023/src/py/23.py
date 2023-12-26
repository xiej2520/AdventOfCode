import sys
import heapq
from collections import defaultdict, Counter, deque
from functools import cache
import math
import bisect

sys.setrecursionlimit(100000000)

lines = []

res = 0

for line in sys.stdin:
    lines.append(line.strip())

m, n = len(lines), len(lines[0])

visited = {}

def dfs(i, j):
    if i == m - 1 and j == n - 2:
        return 0
    visited[(i, j)] = True
    res = -1
    if lines[i][j] == '>':
        if (i, j+1) not in visited:
            res = 1 + dfs(i, j+1)
    elif lines[i][j] == '<':
        if (i, j-1) not in visited:
            res = 1 + dfs(i, j-1)
    elif lines[i][j] == '^':
        if (i-1, j) not in visited:
            res = 1 + dfs(i-1, j)
    elif lines[i][j] == 'v':
        if (i+1, j) not in visited:
            res = 1 + dfs(i+1, j)
    else:
        for d in [(-1, 0), (1, 0), (0, -1), (0, 1)]:
            x = i + d[0]
            y = j + d[1]
            if x < 0 or x >= m or y < 0 or y >= n:
                continue
            if lines[x][y] == '#':
                continue
            if (x, y) not in visited:
                res = max(res, 1 + dfs(x, y))
    del visited[(i, j)]
    return res

res = dfs(0, 1)

print(res)
