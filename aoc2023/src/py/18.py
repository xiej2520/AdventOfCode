import heapq
import sys
from collections import defaultdict, Counter
from functools import cache
import math

sys.setrecursionlimit(100000000)

res = 0

field = [[False for _ in range(600)] for _ in range(600)]

i, j = 150, 150
field[i][j] = True

lines = []
for line in sys.stdin:
    lines.append(line.strip())
    x = line.split(" ")
    dir = x[0]
    l = int(x[1])
    if dir == 'R':
        for k in range(j, j+l+1):
            field[i][k] = True
        j = j + l
    elif dir == 'L':
        for k in range(j-l, j):
            field[i][k] = True
        j = j - l
    elif dir == 'U':
        for k in range(i-l, i):
            field[k][j] = True
        i = i - l
    elif dir == 'D':
        for k in range(i, i+l+1):
            field[k][j] = True
        i = i + l

m, n = len(field), len(field[0])

res = 0

for i in range(m-1):
    parity = 0
    for j in range(n-1):
        if field[i][j]:
            res += 1
            if (field[i-1][j] and field[i+1][j]) or (field[i-1][j] and field[i][j+1]) or (field[i-1][j] and field[i][j-1]):
                parity = parity ^ 1
        else:
            res += parity

for row in field:
    print("".join(map(lambda x: '#' if x else '.', row)))

print(res)
