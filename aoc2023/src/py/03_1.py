import sys
from collections import defaultdict

res = 0
g = []
for line in sys.stdin:
    g.append(line.strip())
m = len(g)
n = len(g[0])

con = [[False for _ in range(n)] for _ in range(m)]
counter = 0

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

z = [[0 for _ in range(n)] for _ in range(m)]

counter = 0

for i in range(m):
    j = 0
    while j < n:
        if g[i][j].isdigit() and con[i][j]:
            l = j
            s = g[i][j]
            con[i][j] = counter
            j += 1
            while j < n and g[i][j].isdigit():
                s += g[i][j]
                con[i][j] = counter
                j += 1
            s = int(s)
            for k in range(l, j):
                z[i][k] = s
            counter += 1
        j += 1

for i in range(m):
    for j in range(n):
        if not g[i][j].isdigit() and g[i][j] != '.':
            l = {}
            def a(x, y):
                if x < 0 or y < 0 or x >= m or y >= n:
                    return
                global z
                if z[x][y] != 0:
                    l[con[x][y]] = z[x][y]
            for dx, dy in dirs:
                a(i + dx, j + dy)
            if len(l) == 2:
                p = 1
                for k in l:
                    p *= l[k]
                res += p

print(res)
