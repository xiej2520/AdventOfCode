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

bricks = []

for line in lines:
    x = line.split('~')
    e1 = list(map(int, x[0].split(',')))
    e2 = list(map(int, x[1].split(',')))
    
    bricks.append([e1, e2])

n = len(bricks)
for b in bricks:
    if b[0][0] > b[1][0]:
        b[0][0], b[1][0] = b[1][0], b[0][0]
    if b[0][1] > b[1][1]:
        b[0][1], b[1][1] = b[1][1], b[0][1]
    if b[0][2] > b[1][2]:
        b[0][2], b[1][2] = b[1][2], b[0][2]
bricks.sort(key=lambda b: b[0][2])
print(bricks)

# x: [0, 9], y: [0, 9], z: [1, 298]
space = [[[-1 for _ in range(300)] for _ in range(10)] for _ in range(10)]
supporting = [{} for _ in range(n)]
supported_by = [{} for _ in range(n)]

for idx, b in enumerate(bricks):
    stop = False
    while not stop:
        if b[0][2] == 1:
            break
        for i in range(b[0][0], b[1][0]+1):
            for j in range(b[0][1], b[1][1]+1):
                if space[i][j][b[0][2] - 1] != -1:
                    stop = True
                    break
            if stop:
                break
        if stop:
            break
        b[0][2] -= 1
        b[1][2] -= 1

    for i in range(b[0][0], b[1][0]+1):
        for j in range(b[0][1], b[1][1]+1):
            if space[i][j][b[0][2] - 1] != -1:
                supporting[space[i][j][b[0][2] - 1]][idx] = True
                supported_by[idx][space[i][j][b[0][2] - 1]] = True

    for i in range(b[0][0], b[1][0]+1):
        for j in range(b[0][1], b[1][1]+1):
            for k in range(b[0][2], b[1][2]+1):
                space[i][j][k] = idx
print(bricks)

print(supporting)

res = 0
for m in supporting:
    only_dep = False
    for supported in m:
        if len(supported_by[supported]) == 1:
            only_dep = True
            break
    if not only_dep:
        res += 1


print(res)
