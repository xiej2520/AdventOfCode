import heapq
import sys
from collections import defaultdict, Counter

lines = []
res = 0

for line in sys.stdin:
    lines.append(line)

adj = defaultdict(lambda: ("",""))

for i in range(2, len(lines)):
    a = lines[i][0:3]
    l = lines[i][7:10]
    r = lines[i][12:15]
    adj[a] = (l, r)

dirs = lines[0].strip()

def l(cur):
    i = 0
    while cur[-1] != 'Z':
        print(cur)
        if dirs[i % len(dirs)] == 'R':
            cur = adj[cur][1]
        else:
            cur = adj[cur][0]
        i += 1
    print(cur, i)
    return i

x = []
for k in adj:
    if k[-1] == 'A':
        x.append(k)

res = 1

def gcd(x, y):
    if y == 0:
        return x
    return gcd(y, x % y)

for i in x:
    y = l(i)
    res *= y / gcd(res, y)

print(res)