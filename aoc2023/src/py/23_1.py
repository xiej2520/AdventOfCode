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
    lines.append(list(line.strip()))

m, n = len(lines), len(lines[0])

lines[0][1] = 'X'
b = {(0, 1): 0}
c = 1
branches = 0
for i in range(m):
    for j in range(n):
        if lines[i][j] == '#':
            continue
        r = 0
        for d in [(-1, 0), (1, 0), (0, -1), (0, 1)]:
            x = i + d[0]
            y = j + d[1]
            if x < 0 or x >= m or y < 0 or y >= n:
                continue
            if lines[x][y] == '#':
                continue
            r += 1
        if r > 2:
            branches += 1
            lines[i][j] = 'X'
            b[(i,j)] = c
            c += 1
            print(i, j, r)

lines[m-1][n-2] = 'X'
b[(m-1, n-2)] = c
c += 1
print(branches)
print(b)
for line in lines:
    print("".join(line))

adj = [{} for _ in range(c)]

def dfs(i, j, s, l, visited):
    visited[i][j] = True
    if lines[i][j] == 'X':
        if b[(i,j)] != s:
            adj[s][b[(i, j)]] = l
        return

    for d in [(-1, 0), (1, 0), (0, -1), (0, 1)]:
        x = i + d[0]
        y = j + d[1]
        if x < 0 or x >= m or y < 0 or y >= n:
            continue
        if lines[x][y] == '#':
            continue
        if not visited[x][y]:
            dfs(x, y, s, l + 1, visited)
    visited[i][j] = False

for p in b:
    visited = [[False for _ in range(n)] for _ in range(m)]
    for d in [(-1, 0), (1, 0), (0, -1), (0, 1)]:
        x = p[0] + d[0]
        y = p[1] + d[1]
        if x < 0 or x >= m or y < 0 or y >= n:
            continue
        if lines[x][y] == '#':
            continue
        if not visited[x][y]:
            dfs(x, y, b[p], 1, visited)
    #dfs(p[0], p[1], b[p], 0, visited)
print(adj)

visited = [False for _ in range(c)]
def dfs(i):
    res = 0
    visited[i] = True
    if i == c-1:
        #print("DONE", visited)
        visited[i] = False
        return 0

    for j in adj[i]:
        if not visited[j]:
            res = max(res, adj[i][j] + dfs(j))
    visited[i] = False
    return res

res = dfs(0)

print(res)
