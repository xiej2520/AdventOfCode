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

print('supporting', supporting)
print('supported by', supported_by)

"""
def chain_react(x):
    ret = 1
    deleted = [False for _ in range(n)]

    q = Queue()
    deleted[x] = True
    q.put(x)
    while not q.empty():
        nq = Queue()
        cur = q.get()
        for next_brick in supporting[cur]:
            supported_now = False
            for support in supported_by[next_brick]:
                if not deleted[support]:
                    #print(x,i,m,"sup",j)
                    supported_now = True
                    break
            if not supported_now:
                ret += 1
                deleted[next_brick] = True
                #print(x,m)
                nq.put(next_brick)
        q = nq
    return ret
"""
def chain_react(x):
    ret = 1
    deleted = [False for _ in range(n)]

    q = {}
    deleted[x] = True
    prev = 0
    q[x] = True
    while prev != len(q):
        nq = q.copy()
        prev = len(q)
        for cur in q:
            for next_brick in supporting[cur]:
                supported_now = False
                for support in supported_by[next_brick]:
                    if not deleted[support]:
                        #print(x,i,m,"sup",j)
                        supported_now = True
                        break
                if not supported_now:
                    ret += 1
                    deleted[next_brick] = True
                    #print(x,m)
                    nq[next_brick] = True
        q = nq

    return len(q)

for i in range(n):
    print(i, "supporting", chain_react(i))

res = 0
for i in range(n):
    res += chain_react(i) - 1


print(res)
