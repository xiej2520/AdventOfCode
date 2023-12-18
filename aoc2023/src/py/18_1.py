import heapq
import sys
from collections import defaultdict, Counter
from functools import cache
import math

sys.setrecursionlimit(100000000)

res = 0


points = [[0,0]]

i, j = 0, 0

lines = []
for line in sys.stdin:
    lines.append(line.strip())
    x = line.split(" ")
    h = x[2]
    l = int(h[2:7],16)
    dir = h[-3]
    if dir == '0':
        dir = 'R'
    elif dir == '1':
        dir = 'D'
    elif dir == '2':
        dir = 'L'
    else:
        dir = 'U'
    #print('dir',dir)
    print(dir, l)

    #print(l)
    #print(i,j)
    res -= l
    if dir == 'R':
        j = j + l
        points.append([i, j])
    elif dir == 'L':
        j = j - l
        points.append([i, j])
    elif dir == 'U':
        i = i - l
        points.append([i, j])
    elif dir == 'D':
        i = i + l
        points.append([i, j])


print(points)
points.pop()
print(points)

for i in range(len(points)):
    p = points[i-1]
    q = points[i]
    res += (p[0] - q[0]) * (p[1] + q[1])

res = int(-res / 2)
res += 1
print(res)
