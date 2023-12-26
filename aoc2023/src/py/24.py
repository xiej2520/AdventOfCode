import sys
import heapq
from collections import defaultdict, Counter, deque
from functools import cache
import math
import bisect

sys.setrecursionlimit(100000000)

lines = []

res = 0

hailstones = []

for line in sys.stdin:
    lines.append(line.strip())
    x = line.split('@')
    hailstones.append([list(map(int, x[0].split(', '))), list(map(int, x[1].split(', ')))])

def intersect(x1,y1,x2,y2,x3,y3,x4,y4):
    if ( (x1-x2)*(y3-y4)-(y1-y2)*(x3-x4) ) == 0:
        return math.inf
    px= ( (x1*y2-y1*x2)*(x3-x4)-(x1-x2)*(x3*y4-y3*x4) ) / ( (x1-x2)*(y3-y4)-(y1-y2)*(x3-x4) ) 
    py= ( (x1*y2-y1*x2)*(y3-y4)-(y1-y2)*(x3*y4-y3*x4) ) / ( (x1-x2)*(y3-y4)-(y1-y2)*(x3-x4) )
    return [px, py]

n = len(hailstones)
for i in range(n):
    for j in range(i+1, n):
        p = hailstones[i]
        q = hailstones[j]
        inter = intersect(p[0][0], p[0][1], p[0][0] + p[1][0], p[0][1] + p[1][1], q[0][0], q[0][1], q[0][0] + q[1][0], q[0][1] + q[1][1])
        if inter == math.inf:
            continue
        if (inter[0] - p[0][0]) / p[1][0] < 0:
            continue
        if (inter[0] - q[0][0]) / q[1][0] < 0:
            continue
        if (inter[1] - p[0][1]) / p[1][1] < 0:
            continue
        if (inter[1] - q[0][1]) / q[1][1] < 0:
            continue

        l = 200000000000000
        h = 400000000000000
        if inter[0] < l or inter[0] > h:
            continue
        if inter[1] < l or inter[1] > h:
            continue

        print(inter)
        res += 1



print(res)
