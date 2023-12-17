import heapq
import sys
from collections import defaultdict, Counter
from functools import cache
import math

sys.setrecursionlimit(100000000)

res = 0

lines = []
for line in sys.stdin:
    lines.append(line.strip())

m, n = len(lines), len(lines[0])

def sim(start):
    tiles = [[False for _ in range(n)] for _ in range(m)]
    occur = [[{} for _ in range(n)] for _ in range(m)]
    q = [start]
    iters = 0
    while q:
        nq = []
        for light in q:
            x, y = light[0], light[1]
            dir = light[2]
            if light[0] < 0 or light[1] < 0 or light[0] >= m or light[1] >= n:
                continue
            if (x, y, dir) in occur[x][y]:
                continue
            occur[x][y][(x,y,dir)] = True
            
            if lines[x][y] == '.':
                tiles[x][y] = True
                if dir == 'u':
                    x -= 1
                elif dir == 'd':
                    x += 1
                elif dir == 'l':
                    y -= 1
                else:
                    y += 1
                nq.append([x, y, dir])
            elif lines[x][y] == '/':
                tiles[x][y] = True
                if dir == 'u':
                    dir = 'r'
                    y += 1
                elif dir == 'd':
                    dir = 'l'
                    y -= 1
                elif dir == 'l':
                    dir = 'd'
                    x += 1
                else:
                    dir = 'u'
                    x -= 1
                nq.append([x, y, dir])
            elif lines[x][y] == '\\':
                tiles[x][y] = True
                if dir == 'u':
                    dir = 'l'
                    y -= 1
                elif dir == 'd':
                    dir = 'r'
                    y += 1
                elif dir == 'l':
                    dir = 'u'
                    x -= 1
                else:
                    dir = 'd'
                    x += 1
                nq.append([x, y, dir])
            elif lines[x][y] == '|':
                tiles[x][y] = True
                nq.extend([[x - 1, y, 'u'], [x + 1, y, 'd']])
            elif lines[x][y] == '-':
                tiles[x][y] = True
                nq.extend([[x, y - 1, 'l'], [x, y + 1, 'r']])

        q = nq
        iters += 1
        if iters > 1000000:
            break

    res = 0
    for row in tiles:
        for t in row:
            if t:
                res += 1
    return res


max_res = 0
for i in range(m):
    max_res = max(max_res, sim([m, 0, 'r']))
for i in range(m):
    max_res = max(max_res, sim([m, n-1, 'l']))
for j in range(n):
    max_res = max(max_res, sim([0, j, 'd']))
for j in range(n):
    max_res = max(max_res, sim([m-1, j, 'u']))

#for r in tiles:
#    print("".join(list(map(lambda x: ('#' if x else '.'), r))))


print(max_res)
