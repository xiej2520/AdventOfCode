import sys
from collections import defaultdict

res = 0
g = []
for line in sys.stdin:
    g.append(line.strip())
m = len(g)
n = len(g[0])

con = [[False for _ in range(n)] for _ in range(m)]

dirs = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)]

def dfs(x, y):
    if x < 0 or y < 0 or x >= m or y >= n:
        return
    if g[x][y].isdigit() and not con[x][y]:
        con[x][y] = True
        dfs(x, y-1)
        dfs(x, y+1)

for i in range(m):
    for j in range(n):
        if not g[i][j].isdigit() and g[i][j] != '.':
            for dx, dy in dirs:
                dfs(i + dx, j + dy)

for i in range(m):
    j = 0
    while j < n:
        if g[i][j].isdigit() and con[i][j]:
            s = g[i][j]
            j += 1
            while j < n and g[i][j].isdigit():
                s += g[i][j]
                j += 1
            res += int(s)
        j += 1

print(res)
