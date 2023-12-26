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
    #lines.append(line.strip())
    x = line.split('@')
    lines.append([list(map(int, x[0].split(', '))), list(map(int, x[1].split(', ')))])
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

        #print(inter)
        res += 1


"""
x + t1 * vx = 237822270988608 + 115 * t1
y + t1 * vy = 164539183264530 + 346 * t1
z + t1 * vz = 381578606559948 + -342 * t1


x + t2 * vx = 287838354624648 + -5 * t2
y + t2 * vy = 284335343503076 + -84 * t2
z + t2 * vz = 181128681512377 + 175 * t2


x + t3 * vx = 341046208911993 + -74 * t3
y + t3 * vy = 120694764237967 + 129 * t3
z + t3 * vz = 376069872241870 + -78 * t3


x + t1 * vx = 237822270988608 + 115 * t1
t1 = (237822270988608 - x) / (vx - 115)
y + t1 * vy = 164539183264530 + 346 * t1
t1 = (164539183264530 - y) / (vy - 346)

(237822270988608 - x) / (vx - 115) = (164539183264530 - y) / (vy - 346)

x + t2 * vx = 287838354624648 + -5 * t2
t2 = (287838354624648 - x) / (vx + 5)
y + t2 * vy = 284335343503076 + -84 * t2
t2 = (284335343503076 - y) / (vy + 84)

(287838354624648 - x) / (vx + 5) = (284335343503076 - y) / (vy + 84)

x + t3 * vx = 341046208911993 + -74 * t3
y + t3 * vy = 120694764237967 + 129 * t3

(341046208911993 - x) / (vx + 74) = (120694764237967 - y) / (vy - 129)

x + t4 * vx = 275834119712623 + 90 * t4
y + t4 * vy = 395388307575057 + -111 * t4

(275834119712623 - x) / (vx - 90) = (395388307575057 - y) / (vy + 111)


"""

from z3 import *

def part2(lines):
    fx,fy,fz,opt = Int("fx"), Int("fy"), Int("fz"), Solver()
    fdx,fdy,fdz = Int("fdx"), Int("fdy"), Int("fdz")
    for i, ((x,y,z), (dx,dy,dz)) in enumerate(lines):
        t = Int(f"t{i}")
        opt.add(t >= 0)
        opt.add(x + dx * t == fx + fdx * t)
        opt.add(y + dy * t == fy + fdy * t)
        opt.add(z + dz * t == fz + fdz * t)
    assert str(opt.check()) == 'sat'
    print(opt.model().eval(fx),opt.model().eval(fy),opt.model().eval(fz))
    return opt.model().eval(fx + fy + fz)

print(part2(lines))
