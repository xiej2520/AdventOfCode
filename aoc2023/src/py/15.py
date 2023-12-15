import heapq
import sys
from collections import defaultdict, Counter
from functools import cache
import math

sys.setrecursionlimit(100000000)

lines = []
for line in sys.stdin:
    lines.append(line.strip())

m, n = len(lines), len(lines[0])

sp = lines[0].split(",")

boxes = [[] for _ in range(256)]

res = 0
for s in sp:
    v = 0
    for c in s:
        v += ord(c)
        v = v * 17
        v = v % 256
    res += v

for s in sp:
    if s[-1] == '-':
        t = True
        for i in range(256):
            for j in range(len(boxes[i])):
                if boxes[i][j][0] == s[:-1]:
                    del boxes[i][j]
                    t = False
                    break
            if not t:
                break
    else:
        x = s.split("=")
        l = x[0]
        v = 0
        for c in l:
            v += ord(c)
            v = v * 17
            v = v % 256

        focal = int(x[1])
        found = False
        for j in range(len(boxes[v])):
            if boxes[v][j][0] == l:
                boxes[v][j][1] = focal
                found = True
                break
        if not found:
            boxes[v].append([l, focal])

#    print(boxes)
#    print()
print(boxes)

res_2 = 0
for i in range(256):
    for j in range(len(boxes[i])):
        print((1 + i) * (1 + j) * boxes[i][j][1])
        res_2 += (1 + i) * (1 + j) * boxes[i][j][1]

print(res)
print(res_2)
