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
    
    r = []
    s = []
    for i in range(5):
        r.extend(record)
        r.append("?")
        s.extend(size)
    r.pop()
    
    n = len(r)
    m = len(s)
    #print(r)
    #print(s)
    @cache
    def dfs(i, j):
        if i + s[j] > n:
            return 0

        for k in r[i:i+s[j]]:
            if k == '.':
                return 0

        if i + s[j] < n and r[i+s[j]] == '#':
            return 0

        if j == m - 1:
            for k in r[i+s[j]+1:]:
                if k == '#':
                    return 0
            return 1
        
        ret = 0
        for k in range(i+s[j]+1, n):
            if r[k] != '.':
                ret += dfs(k, j + 1)
            if r[k] == '#':
                break
        return ret

    ways = 0
    for i in range(0, n):
        if r[i] != '.':
            ways += dfs(i, 0)
        if r[i] == '#':
            break
    print(ways)
    counts += ways

print(counts)
