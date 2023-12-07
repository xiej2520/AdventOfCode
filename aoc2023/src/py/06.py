import sys
from collections import defaultdict

res = 99999999999
lines = []
for line in sys.stdin:
    lines.append(line)

times = list(map(int, lines[0].split(":")[1].split()))
dists = list(map(int, lines[1].split(":")[1].split()))

n = len(times)
ways = []
for i in range(n):
    w = 0
    for t in range(0, times[i]+1):
        if t * (times[i] - t) > dists[i]:
            w += 1
    ways.append(w)

print(times, dists)

print(ways)

r = 1
for i in ways:
    r *= i
print(r)
