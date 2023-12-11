import heapq
import sys
from collections import defaultdict, Counter

sys.setrecursionlimit(100000000)

lines = []
for line in sys.stdin:
    lines.append(line.strip())

m, n = len(lines), len(lines[0])

d = {}

h = 0
for i in range(m):
    if all(map(lambda x: x == '.', lines[i])):
        h += 999999
    for j in range(n):
        if lines[i][j] == '#':
            d[(i,j)] = [i + h, 0]

h = 0
for j in range(n):
    y = True
    for i in range(m):
        if lines[i][j] != '.':
            y = False
    if y:
        h += 999999
    for i in range(n):
        if lines[i][j] == '#':
            d[(i,j)][1] = j + h

res = 0

for k in d:
    for l in d:
        res += abs(d[k][0] - d[l][0]) + abs(d[k][1] - d[l][1])
print(res // 2)
