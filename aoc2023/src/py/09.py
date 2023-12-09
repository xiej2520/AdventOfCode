import heapq
import sys
from collections import defaultdict, Counter

lines = []
res = 0

def allzero(l):
    for i in l:
        if i != 0:
            return False
    return True

for line in sys.stdin:
    lines.append(line)
    nums = list(map(int, line.split()))
    diffs = []
    cur = nums[:]
    while not allzero(cur):
        diffs.append(cur)
        x = []
        for i in range(1, len(cur)):
            x.append(cur[i] - cur[i-1])
        cur = x
    diffs.append(cur)
    print(diffs)
    next = 0
    for i in range(len(diffs)-2, -1, -1):
        next = next + diffs[i][-1]
    print(next)
    res += next
print(res)



