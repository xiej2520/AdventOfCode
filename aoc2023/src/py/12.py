import heapq
import sys
from collections import defaultdict, Counter
from functools import lru_cache, cache
import math

sys.setrecursionlimit(100000000)

counts = 0

lines = []
for line in sys.stdin:
    lines.append(line.strip())
    x = line.split()
    record = list(x[0])
    size = list(map(int, x[1].split(",")))

    n = len(record)
    ways = 0

    def is_valid():
        groups = []
        i = 0
        while i < n:
            if record[i] == '#':
                l = i
                while i < n and record[i] == '#':
                    i += 1
                groups.append(i - l)
            else:
                i += 1
        return groups == size

    def dfs(i):
        if i == n:
            if is_valid():
                global ways
                ways += 1
            return
        if record[i] != '?':
            dfs(i + 1)
        else:
            record[i] = '#'
            dfs(i + 1)
            record[i] = '.'
            dfs(i + 1)
            record[i] = '?'
    
    dfs(0)
    counts += ways

    print("ways", ways)

print(counts)
