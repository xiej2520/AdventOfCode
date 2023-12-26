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
            si = i
            sj = j

grid = []
dup = 155
for k in range(dup):
    for i in range(m):
        grid.append(list(lines[i].replace('S', '.')))
        for j in range(dup-1):
            grid[-1].extend(list(lines[i].replace('S', '.')))

si = si + dup // 2 * m
sj = sj + dup // 2 * n

grid[si][sj] = 'S'
m = len(grid)
n = len(grid[0])

#for line in grid:
#    print("".join(line))
lines = grid


# 131 x 131 grid
# Vertical and horizontal of 'S' are all '.'


def num_filled(N):
    q = [(si, sj)]
    for k in range(N):
        print(N,k)
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
    #x = grid[:]
    #for (i, j) in q:
    #    x[i][j] = 'O'
    #print()
    #for line in x:
    #    print("".join(line))
    #print()
    return len(q)


#n = 26501365
N = 4

"""
6: 43 reachable, 13 wide, every other
7: 56 reachable, 15 wide, 

26501365
131x131 input, quadratic fit
remainder 65 mod 262

65 2722
327 71435
589 231894

469343767311222 is too low


65 3719
327 91987
589 297559

600090522932119 is correct...
"""


#grid = [list(line) for line in lines]
#for (i, j) in q:
#    grid[i][j] = 'O'

#for i in range(m):
#    if grid[i][sj] == '.':
#        grid[i][sj] = '$'
#for j in range(n):
#    if grid[si][j] == '.':
#        grid[si][j] = '$'

print(262+65, num_filled(262+65))
print(262+262+65, num_filled(262+262+65))
print(262+262+262+65, num_filled(262+262+262+65))
#for i in range(1,1000):
#    print(i, num_filled(i))
